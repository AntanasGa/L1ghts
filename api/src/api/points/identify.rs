use std::time::Duration;
use actix_web::web;
use diesel::prelude::*;

use crate::{
    types::DbPool,
    models::{
        QueryById,
        Points,
    },
    api::{
        ApiError,
        helpers::{
            props::LIGHT_LEVEL_MAX,
            batcher::Batcher,
        },
    },
};



pub async fn post(
    pool: web::Data<DbPool>,
    data: web::Json<QueryById>,
    batcher: web::Data<Batcher>,
) -> Result<web::Json<QueryById>, ApiError> {
    let point_id = data.id;
    let initiator_pool = pool.clone();
    let point_list = web::block(move || {
        let mut con = initiator_pool.get()
        .map_err(|err| {
            log::error!("Failed to get pool: {}", err);
            ApiError::InternalErr
        })?;
        use crate::schema::points::dsl::{
            points,
            id as p_id,
            val
        };
        let point_list = points.load::<Points>(& mut con)
        .map_err(|err| {
            log::error!("Failed fetching points: {}", err);
            ApiError::InternalErr
        })?;

        if !point_list.iter().any(|v| &v.id == &point_id) {
            return Err(ApiError::BadRequest);
        }

        diesel::update(points).set(val.eq(0)).execute(& mut con)
        .map_err(|err| {
            log::error!("Failed setting points to 0: {}", err);
            ApiError::InternalErr
        })?;

        diesel::update(points).filter(p_id.eq(point_id.clone()))
        .set(val.eq(LIGHT_LEVEL_MAX))
        .execute(& mut con)
        .map_err(|err| {
            log::error!("Failed updating point value: {}", err);
            ApiError::InternalErr
        })?;
        Ok(point_list)
    })
    .await
    .map_err(|err| {
        log::error!("Identify block failed: {}", err);
        ApiError::InternalErr
    })??;

    batcher.request();
    actix_web::rt::time::sleep(Duration::from_secs(2)).await;

    web::block(move || {
        let mut con = pool.get()
        .map_err(|err| {
            log::error!("Failed to get pool: {}", err);
            ApiError::InternalErr
        })?;
        use crate::schema::points::dsl::*;
        for item in &point_list {
            diesel::update(points).filter(id.eq(item.id))
            .set(val.eq(item.val))
            .execute(&mut con)
            .map_err(|err| {
                log::error!("Failed to update point [{}]: {}", item.id, err);
                ApiError::InternalErr
            })?;
        }
        Ok(())
    })
    .await
    .map_err(|err| {
        log::error!("Identify recover block failed: {}", err);
        ApiError::InternalErr
    })??;
    batcher.request();

    let result = QueryById { id: point_id.clone() };
    Ok(web::Json(result))
}
