use std::sync::Arc;

use actix_web::web;

use diesel::prelude::*;

use diesel::update;

use crate::api::helpers::i2c::LightDevices;
use crate::models::Points;
use crate::types::SharedStorage;
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

    let i2cid = Arc::try_unwrap(shared_data.i2c_device.clone())
    .map_err(|err| {
        log::error!("Failed fetching i2c identifier: {}", err);
        *modify_lock = false;
        ApiError::InternalErr
    })?;
    let mut controller = LightDevices::new(i2cid)
    .map_err(|err| {
        log::error!("Failed to get i2c driver: {}", err);
        *modify_lock = false;
        ApiError::InternalErr
    })?;

    let mut con = pool.get()
    .map_err(|err| {
        log::error!("Failed to get pool: {}", err);
        ApiError::InternalErr
    })?;
    let uid = token.claims.uid.clone();
    let active_id = data.id.clone();
    let detached: Result<Vec<Points>, ApiError> = web::block(move || {
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
        *modify_lock = false;
        ApiError::InternalErr
    })?;

    let updated_points = detached.map_err(|err| {
        *modify_lock = false;
        err
    })?;

    let converted = LightDevices::convert_points(updated_points.clone(), false);

    let mut dev_con = pool.get()
    .map_err(|err| {
        log::error!("Failed to get pool_points: {}", err);
        *modify_lock = false;
        ApiError::InternalErr
    })?;

    use crate::schema::devices::dsl::*;
    let db_device_query: Result<Vec<(i32, i32)>, ApiError> = web::block(move || {
        devices.select((id, adr)).load::<(i32, i32)>(&mut dev_con)
        .map_err(|err| {
            log::error!("Requesting Data failed: {}", err);
            ApiError::InternalErr
        })
    })
    .await
    .map_err(|err| {
        log::error!("Point update block failed: {}", err);
        *modify_lock = false;
        ApiError::InternalErr
    })?;

    let db_devices = db_device_query.map_err(|err| {
        *modify_lock = false;
        err
    })?;

    for (dev_id, dev_adr) in db_devices {
        match converted.iter().position(|(k, _)| k.clone() == dev_id) {
            Some(index) => {
                controller.set_light_levels(dev_adr as u16, converted[index].1.clone())
                    .map_err(|err| {
                        log::error!("Point update failed at set_light_levels for {} : {}", dev_adr, err);
                        *modify_lock = false;
                        ApiError::InternalErr
                    })?;
            },
            None => (),
        }
    }

    *modify_lock = false;

    let result = QueryById { id: active_id };
    Ok(web::Json(result))
}
