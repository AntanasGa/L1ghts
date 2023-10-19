use crate::api::ApiError;
use crate::api::helpers::db::points::{
    fill_diff,
    reduce_diff,
};
use crate::api::helpers::i2c::LightDevices;
use crate::types::{
    DbPool,
    SharedStorage,
};
use crate::models::{
    Devices,
    NewDevices,
    Points,
};
use actix_web::web;
use diesel::{
    prelude::*,
    insert_into,
    delete,
};

// fetch from database
pub async fn get(pool: web::Data<DbPool>) -> Result<web::Json<Vec<Devices>>, ApiError> {
    let pool_device = pool.clone();
    let device_request = web::block(move || {
        use crate::schema::devices::dsl::*;
        let mut con = pool_device.get()
        .map_err(|err| {
            log::error!("Could not fetch connection from pool: {}", err);
            ApiError::InternalErr
        })?;
        devices.load::<Devices>(&mut con)
        .map_err(|err| {
            log::error!("Fetching devices failed: {}", err);
            ApiError::InternalErr
        })
    })
    .await
    .map_err(|err| {
        log::error!("Web block failed with: {}", err);
        ApiError::InternalErr
    })??;
    Ok(web::Json(device_request))
}

// updates from i2c and refreshes the 
pub async fn post(pool: web::Data<DbPool>, shared_data: web::Data<SharedStorage>) -> Result<web::Json<Vec<Devices>>, ApiError> {
    let i2cid = *shared_data.i2c_device.clone();
    let mut controller = LightDevices::new(i2cid)
    .map_err(|err| {
        log::error!("Failed to get i2c driver: {}", err);
        ApiError::TooManyRequests
    })?;
    
    let detected_devices = controller.controllers()
    .map_err(|err| {
        log::error!("Failed to fetch i2c controllers: {}", err);
        ApiError::InternalErr
    })?;

    use crate::schema::devices::dsl::*;
    let pool_read_devices = pool.clone();
    let db_devices = web::block(move || {
        let mut con = pool_read_devices.get()
        .map_err(|err| {
            log::error!("Could not fetch connection from pool_read_devices: {}", err);
            ApiError::InternalErr
        })?;
        devices.load::<Devices>(&mut con)
        .map_err(|err| {
            log::error!("Requesting Data failed: {}", err);
            ApiError::InternalErr
        })
    })
    .await
    .map_err(|err| {
        log::error!("Web block device read failed with: {}", err);
        ApiError::InternalErr
    })??;
    // dont bother when there's nothing to do
    if db_devices.len() == 0 && detected_devices.len() == 0 {
        return Ok(web::Json(db_devices));
    }
    // detection block
    // sort indexes into updatable and insertable
    let mut address_update: Vec<(usize, usize)> = vec![];
    let mut address_delete: Vec<usize> = vec![];
    let mut address_insert: Vec<usize> = vec![];
    let mut no_insert: Vec<usize> = vec!();
    for (db_index, device) in db_devices.iter().enumerate() {
        match detected_devices.iter().position(|r| r.adr == device.adr) {
            Some(detected_index) => {
                no_insert.push(detected_index);
                let matched_device = &detected_devices[detected_index];
                if matched_device.endpoint_count != device.endpoint_count {
                    address_update.push((db_index, detected_index));
                    continue;
                }
            },
            None => {
                address_delete.push(db_index);
            },
        };
    }
    for detected_index in 0..detected_devices.len() {
        match no_insert.iter().position(|r| r == &detected_index) {
            Some(_) => (),
            None => {
                address_insert.push(detected_index);
            },
        }
    }

    // db device insert block
    let pool_insert = pool.clone();
    let detected_devs = detected_devices.clone();
    web::block(move || {
        let mut con = pool_insert.get()
        .map_err(|err| {
            log::error!("Could not fetch connection from pool_insert: {}", err);
            ApiError::InternalErr
        })?;
        let insertion: Vec<NewDevices> = address_insert.iter()
        .map(|v| {
            detected_devs[*v].to_owned()
        })
        .collect();
        insert_into(devices).values(insertion).execute(&mut con)
        .map_err(|err| {
            log::error!("Failed to insert new devices: {}", err);
            ApiError::InternalErr
        })
    })
    .await
    .map_err(|err| {
        log::error!("Web block for device inserting failed with: {}", err);
        ApiError::InternalErr
    })??;

    // db device update block
    let pool_update = pool.clone();
    let db_devs = db_devices.clone();
    web::block(move || {
        let mut con = pool_update.get()
        .map_err(|err| {
            log::error!("Could not fetch connection from pool_update: {}", err);
            ApiError::InternalErr
        })?;
        let updated_devices = address_update.iter()
        .map(|sec| {
            let mut devc_clone = db_devs[sec.0].clone();
            devc_clone.endpoint_count = detected_devices[sec.0].endpoint_count.clone();
            devc_clone
        })
        .collect::<Vec<Devices>>();
        for devc_update in updated_devices {
            diesel::update(devices).filter(crate::schema::devices::dsl::id.eq(devc_update.id))
            .set((
                crate::schema::devices::dsl::endpoint_count.eq(devc_update.endpoint_count),
            ))
            .execute(&mut con)
            .map_err(|err| {
                log::error!("Failed to update device id [{}]: {},", devc_update.id, err);
                ApiError::InternalErr
            })?;
        }
        Ok(true)
    })
    .await
    .map_err(|err| {
        log::error!("Web block for device updating failed with: {}", err);
        ApiError::InternalErr
    })??;
    
    // db device delete block
    let pool_delete = pool.clone();
    if address_delete.len() > 0 {
        web::block(move || {
            let mut con = pool_delete.get()
            .map_err(|err| {
                log::error!("Could not fetch connection from pool_update: {}", err);
                ApiError::InternalErr
            })?;
            let mut delete_ = delete(devices).into_boxed();
            let mut first = true;
            for db_id in address_delete {
                let i32_db_id = db_id as i32;
                if first {
                    first = false;
                    delete_ = delete_.filter(id.eq(i32_db_id));
                } else {
                    delete_ = delete_.or_filter(id.eq(i32_db_id));
                }
            }
            delete_.execute(&mut con)
            .map_err(|err| {
                log::error!("Failed to update device id: {},", err);
                ApiError::InternalErr
            })
        })
        .await
        .map_err(|err| {
            log::error!("Web block for device deleting failed with: {}", err);
            ApiError::InternalErr
        })??;
    }

    // picking up all updates
    let pool_rebase = pool.clone();
    let rebase_db_devices = web::block(move || {
        let mut con = pool_rebase.get()
        .map_err(|err| {
            log::error!("Could not fetch connection from pool_rebase: {}", err);
            ApiError::InternalErr
        })?;
        devices.load::<Devices>(&mut con)
        .map_err(|err| {
            log::error!("Fetching devices failed: {}", err);
            ApiError::InternalErr
        })
    })
    .await
    .map_err(|err| {
        log::error!("Web block for device deleting failed with: {}", err);
        ApiError::InternalErr
    })??;

    // updating points for devices
    let rebase_db_devs = rebase_db_devices.clone();
    let pool_rebase_point = pool.clone();
    web::block(move || {
        let mut con = pool_rebase_point.get()
        .map_err(|err| {
            log::error!("Could not fetch connection from pool_rebase: {}", err);
            ApiError::InternalErr
        })?;
        for devc in rebase_db_devs {
            use crate::schema::points::dsl::*;
            let db_points = points.filter(device_id.eq(devc.id))
            .load::<Points>(&mut con)
            .map_err(|err| {
                log::error!("Fetching devices failed: {}", err);
                ApiError::InternalErr
            })?;
            let point_count = devc.endpoint_count as usize;

            if db_points.len() == point_count {
                continue;
            }

            let diff = db_points.len().abs_diff(point_count);
            if db_points.len() < point_count {
                let max_value = match db_points.iter().map(|point| point.device_position).max() {
                    Some(v) => v,
                    None => -1
                };
                fill_diff(&mut con, diff, devc.id, max_value)?;
                continue;
            }

            if db_points.len() > point_count {
                reduce_diff(&mut con, diff, db_points)?;
            }
        }
        Ok(true)
    })
    .await
    .map_err(|err| {
        log::error!("Web block for device point updating failed with: {}", err);
        ApiError::InternalErr
    })??;
    Ok(web::Json(rebase_db_devices))
}
