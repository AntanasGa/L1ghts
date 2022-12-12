pub mod identify;

use crate::{
    api::ApiError,
    types::{
        DbPool,
        SharedStorage,
    },
    models::{
        Points,
        PointsUpdate,
    },
    api::helpers::props::{
        LIGHT_LEVEL_MAX,
        LIGHT_LEVEL_MIN,
        ROTATION_MIN,
        ROTATION_MAX,
    },
};
use actix_web::{
    web,
};
use diesel::{prelude::*, update};

pub async fn get(pool: web::Data<DbPool>) -> Result<web::Json<Vec<Points>>, ApiError> {
    let pool_points = pool.clone();
    let response = web::block(move || {
        let mut con = pool_points.get()
        .map_err(|err| {
            log::error!("Failed to get pool_points: {}", err);
            ApiError::InternalErr
        })?;
        use crate::schema::points::dsl::*;
        points.order(id.asc())
        .load::<Points>(&mut con)
        .map_err(|err| {
            log::error!("Fetching points failed: {}", err);
            ApiError::InternalErr
        })
    })
    .await
    .map_err(|err| {
        log::error!("Point block failed: {}", err);
        ApiError::InternalErr
    })??;
    Ok(web::Json(response))
}

pub async fn put(
    pts: web::Json<Vec<Points>>,
    pool: web::Data<DbPool>,
    shared_data: web::Data<SharedStorage>,
) -> Result<web::Json<Vec<Points>>, ApiError> {
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
    let mut con = pool.get()
    .map_err(|err| {
        log::error!("Failed to get pool_points: {}", err);
        ApiError::InternalErr
    })?;
    let detached = web::block(move || {
        use crate::schema::points::dsl::*;
        for item in pts.iter() {
            diesel::update(points)
            .filter(id.eq(item.id))
            .filter(device_id.eq(item.device_id))
            .set(&PointsUpdate {
                active: Some(item.active),
                tag: Some(item.tag.clone()),
                width: Some(if item.width >= 0.0 { item.width } else { 0.0 }),
                height: Some(if item.height >= 0.0 { item.height } else { 0.0 }),
                x: Some(if item.x >= 0.0 { item.x } else { 0.0 }),
                y: Some(if item.y >= 0.0 { item.y } else { 0.0 }),
                watts: Some(if item.watts >= 0.0 { item.watts } else { 0.0 }),
                val: Some(if item.val > LIGHT_LEVEL_MAX {
                        LIGHT_LEVEL_MAX
                    } else if item.val < LIGHT_LEVEL_MIN {
                        LIGHT_LEVEL_MIN
                    } else {
                        item.val
                    }),
                rotation: Some(if item.rotation < ROTATION_MIN {
                        ROTATION_MIN
                    } else if item.rotation >= ROTATION_MAX {
                        ROTATION_MAX
                    } else {
                        item.rotation
                    }),
            })
            .execute(&mut con)
            .map_err(|err| {
                log::error!("updating [{}] point failed: {}", item.id , err);
                ApiError::InternalErr
            })?;
        }
        use crate::schema::presets::dsl::{
            presets,
            active,
        };
        update(presets).filter(active.eq(true))
        .set(active.eq(false))
        .execute(&mut con)
        .map_err(|err| {
            log::error!("failed to update presets: {}", err);
            ApiError::InternalErr
        })?;
        points.order(id.asc())
        .load::<Points>(&mut con)
        .map_err(|err| {
            log::error!("Fetching points failed: {}", err);
            ApiError::InternalErr
        })
        // TODO: add i2c routine
    })
    .await
    .map_err(|err| {
        log::error!("Point update block failed: {}", err);
        ApiError::InternalErr
    });

    *modify_lock = false;
    let result = detached??;

    Ok(web::Json(result))
}
