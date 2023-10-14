use actix_web::web;
use diesel::prelude::*;

use crate::{
  types::{
    DbPool,
    SharedStorage,
  },
  models::{
    SinglePoint,
    Points,
  },
  api::{
    ApiError,
    helpers::{
    i2c::LightDevices,
    props::{
      LIGHT_LEVEL_MAX,
      LIGHT_LEVEL_MIN,
    },
  },
},
};



pub async fn get(
  path: web::Path<i32>,
  pool: web::Data<DbPool>,
) -> Result<web::Json<SinglePoint>, ApiError> {
  let point_id = path.into_inner();
  let pool_points = pool.clone();
  let response = web::block(move || {
      let mut con = pool_points.get()
      .map_err(|err| {
          log::error!("Failed to get pool_points: {}", err);
          ApiError::InternalErr
      })?;
      use crate::schema::points::dsl::*;
      points.order(id.asc())
      .filter(id.eq(point_id))
      .limit(1)
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

    match response.first() {
        Some(v) => Ok(web::Json(SinglePoint { value: v.val })),
        None => Err(ApiError::NotFound),
    }
}

pub async fn put(
  path: web::Path<i32>,
  pool: web::Data<DbPool>,
  data: web::Json<SinglePoint>,
  shared_data: web::Data<SharedStorage>,
) -> Result<web::Json<SinglePoint>, ApiError> {

  let point_id = path.into_inner();

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
      *modify_lock = false;
      ApiError::InternalErr
    })?;

    let i2cid = *shared_data.i2c_device.clone();
    let mut controller = LightDevices::new(i2cid)
    .map_err(|err| {
      log::error!("Failed to get i2c driver: {}", err);
      *modify_lock = false;
      ApiError::InternalErr
    })?;

    let new_value = if data.value > LIGHT_LEVEL_MAX {
      LIGHT_LEVEL_MAX
    } else if data.value < LIGHT_LEVEL_MIN {
        LIGHT_LEVEL_MIN
    } else {
        data.value
    };

    let detached: Result<Vec<Points>, ApiError> = web::block(move || {
      use crate::schema::points::dsl::*;
      diesel::update(points).filter(id.eq(point_id)).set(val.eq(new_value)).execute(&mut con).map_err(|err| {
          log::error!("updating single [{}] point failed: {}", point_id, err);
          ApiError::InternalErr
      })?;

      use crate::schema::presets::dsl::{
          presets,
          active,
      };
      diesel::update(presets).filter(active.eq(true))
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
    })
    .await
    .map_err(|err| {
        log::error!("Point update block failed: {}", err);
        *modify_lock = false;
        ApiError::InternalErr
    })?;

    let result = detached.map_err(|err| {
      // unblocking i2c
      *modify_lock = false;
      err
    })?;

    let converted = LightDevices::convert_points(result.clone(), false);

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
  Ok(web::Json(SinglePoint { value: new_value }))
}
