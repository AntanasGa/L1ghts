use std::{
    sync::Arc,
    time::Duration,
    thread
};

use actix_web::web;
use diesel::prelude::*;

use crate::{
    types::{
        DbPool,
        SharedStorage,
    },
    models::{
        QueryById,
        Points,
        Devices,
    },
    api::{
        ApiError,
        helpers::{
        i2c::LightDevices,
        props::LIGHT_LEVEL_MAX,
    },
},
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
        let device = devices.filter(id.eq(point[0].device_id))
        .load::<Devices>(&mut con)
        .map_err(|err| {
            log::error!("Failed to point: {}", err);
            ApiError::InternalErr
        })?;
        if device.len() != 1 {
            log::error!("Could not find the specific device with device.id [{}], found count: [{}]", point[0].device_id, device.len());
            return Err(ApiError::InternalErr);
        }

        // for identifying we only need 1 device to be updated
        let current_lights = controller.get_light_levels(device[0].adr as u16)
        .map_err(|err| {
            log::error!("failed fetching light levels: {}", err);
            ApiError::InternalErr
        })?;
        let test_lights: Vec<i32> = (0..device[0].endpoint_count).into_iter()
        .map(|v| match v == point[0].device_position {
            true => LIGHT_LEVEL_MAX,
            false => 0,
        }).collect();

        controller.set_light_levels(device[0].adr as u16, test_lights)
        .map_err(|err| {
            log::error!("failed to set identifier setting: {}", err);
            ApiError::InternalErr
        })?;

        // delay of 2 secs for identifying
        thread::sleep(Duration::from_secs(2));
        controller.set_light_levels(device[0].adr as u16, current_lights)
        .map_err(|err| {
            log::error!("failed to set back light levels: {}", err);
            ApiError::InternalErr
        })?;
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
