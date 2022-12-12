use actix_web::web;
use diesel::prelude::*;

use crate::{
    types::{
        DbPool,
        SharedStorage,
    },
    models::{QueryById, Points, Devices},
    api::ApiError,
};



pub async fn post(
    pool: web::Data<DbPool>,
    data: web::Json<QueryById>,
    shared_data: web::Data<SharedStorage>,
) -> Result<web::Json<QueryById>, ApiError> {
    match shared_data.light_update_lock.try_read() {
        Ok(v) => {
            if *v == true {
                drop(v);
                return Err(ApiError::TooManyRequests);
            }
            drop(v);
            Ok(())
        },
        Err(e) => {
            log::error!("Could not read light_update_lock: {}", e);
            Err(ApiError::InternalErr)
        },
    }?;
    let mut modify_lock = match shared_data.light_update_lock.try_write() {
        Ok(v) => Ok(v),
        Err(e) => {
            log::error!("Could not fetch writable light_update_lock: {}", e);
            Err(ApiError::InternalErr)
        },
    }?;

    *modify_lock = true;
    let point_id = data.id;

    let detatched = web::block(move || {
        let mut con = pool.get()
        .map_err(|err| {
            log::error!("Failed to get pool: {}", err);
            ApiError::InternalErr
        })?;
        use crate::schema::points::dsl::{
            points,
            id as p_id,
        };
        let point = points.filter(p_id.eq(point_id.clone()))
        .load::<Points>(&mut con)
        .map_err(|err| {
            log::error!("Failed to point: {}", err);
            ApiError::InternalErr
        })?;
        if point.len() > 1 {
            log::error!("Too many point entries with id:{}", point_id.clone());
            return Err(ApiError::InternalErr);
        }
        if point.len() < 1 {
            return Err(ApiError::Conflict);
        }
        use crate::schema::devices::dsl::*;
        let _device = devices.filter(id.eq(point[0].device_id))
        .load::<Devices>(&mut con)
        .map_err(|err| {
            log::error!("Failed to point: {}", err);
            ApiError::InternalErr
        })?;
        // TODO: map identify with delay of 2 secs
        Ok(())
    })
    .await
    .map_err(|err| {
        log::error!("Identify block failed: {}", err);
        ApiError::InternalErr
    });
    
    *modify_lock = false;
    
    detatched??;

    let result = QueryById { id: point_id.clone() };
    Ok(web::Json(result))
}
