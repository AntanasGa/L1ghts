use actix_web::web;

use diesel::prelude::*;

use diesel::update;

use crate::api::helpers::batcher::Batcher;
use crate::models::Points;
use crate::{
    types::DbPool,
    middleware::auth::TokenData,
    api::ApiError,
    models::{
        QueryById,
        Presets,
        PresetItems,
    },
};


pub async fn get(
    pool: web::Data<DbPool>,
    token: web::ReqData<TokenData>,
) -> Result<web::Json<QueryById>, ApiError> {
    let uid = token.claims.uid;

    let active_id = web::block(move || {
        let mut con = pool.get()
        .map_err(|err| {
            log::error!("Failed to get pool: {}", err);
            ApiError::InternalErr
        })?;
        use crate::schema::presets::dsl::*;
        let current = presets.select(id)
        .filter(active.eq(true).and(user_id.eq(uid)))
        .load::<i32>(&mut con)
        .map_err(|err| {
            log::error!("Failed to fetch active preset: {}", err);
            ApiError::InternalErr
        })?;
        if current.len() > 1 {
            // should never be more than one preset active at a time
            return Err(ApiError::InternalErr);
        }
        if current.len() < 1 {
            return Ok(-1);
        }
        return Ok(current[0]);
    })
    .await
    .map_err(|err| {
        log::error!("Preset fetching block failed: {}", err);
        ApiError::InternalErr
    })??;
    let response = QueryById {
        id: active_id,
    };
    Ok(web::Json(response))
}

pub async fn put(
    pool: web::Data<DbPool>,
    token: web::ReqData<TokenData>,
    data: web::Json<QueryById>,
    batcher: web::Data<Batcher>,
) -> Result<web::Json<QueryById>, ApiError> {
    let mut con = pool.get()
    .map_err(|err| {
        log::error!("Failed to get pool: {}", err);
        ApiError::InternalErr
    })?;
    let uid = token.claims.uid.clone();
    let active_id = data.id.clone();
    web::block(move || {
        use crate::schema::presets::dsl::*;
        let selected_presets = presets.filter(id.eq(active_id.clone()).and(user_id.eq(uid.clone())))
        .load::<Presets>(&mut con)
        .map_err(|err| {
            log::error!("Failed to fetch user related preset: {}", err);
            ApiError::InternalErr
        })?;

        if selected_presets.len() > 1 {
            return Err(ApiError::InternalErr);
        }
        if selected_presets.len() < 1 {
            return Err(ApiError::Conflict);
        }

        update(presets).filter(active.eq(true))
        .set(active.eq(false))
        .execute(&mut con)
        .map_err(|err| {
            log::error!("Failed to fetch set presets as inactive: {}", err);
            ApiError::InternalErr
        })?;

        update(presets).filter(id.eq(active_id.clone()))
        .set(active.eq(true))
        .execute(&mut con)
        .map_err(|err| {
            log::error!("Failed to set active preset: {}", err);
            ApiError::InternalErr
        })?;

        use crate::schema::preset_items::dsl::{
            preset_items,
            preset_id,
        };
        let selected_preset_items = preset_items.filter(preset_id.eq(active_id.clone()))
        .load::<PresetItems>(&mut con)
        .map_err(|err| {
            log::error!("Failed to fetch selected preset items: {}", err);
            ApiError::InternalErr
        })?;

        use crate::schema::points::dsl::{
            points,
            id as point_table_id,
            val as point_table_val,
        };
        for p_item in &selected_preset_items {
            update(points).filter(point_table_id.eq(p_item.point_id))
            .set(point_table_val.eq(p_item.val))
            .execute(&mut con)
            .map_err(|err| {
                log::error!("Failed to update point [{}]: {}", p_item.point_id, err);
                ApiError::InternalErr
            })?;
        }
        points.order(point_table_id.asc())
        .load::<Points>(&mut con)
        .map_err(|err| {
            log::error!("Fetching points failed: {}", err);
            ApiError::InternalErr
        })
    })
    .await
    .map_err(|err| {
        log::error!("Preset activating block failed: {}", err);
        ApiError::InternalErr
    })??;

    batcher.request();

    let result = QueryById { id: active_id };
    Ok(web::Json(result))
}
