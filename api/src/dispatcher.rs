use crate::api::helpers::i2c::LightDevices;
use crate::types::DbPool;
use crate::models::Points;
use diesel::prelude::*;

pub async fn dispatch(db_pool: DbPool, i2c_device_id: u8) {
  let mut con = match db_pool.get() {
      Ok(r) => r,
      Err(e) => {
        log::error!("Dispatcher failed to fetch db_pool: {}", e);
        return;
      }
  };

  use crate::schema::points::dsl::*;
  let point_list = match points.order(id.asc()).load::<Points>(&mut con) {
    Ok(v) => v,
    Err(e) => {
      log::error!("Dispatcher fetching points failed: {}", e);
      return;
    },
  };

  use crate::schema::devices::dsl::{
    devices,
    adr,
    id as device_id,
  };

  let db_devices: Vec<(i32, i32)> = match devices.select((device_id, adr)).load::<(i32, i32)>(&mut con) {
    Ok(v) => v,
    Err(e) => {
      log::error!("Dispatcher fetching db devices failed: {}", e);
      return;
    },
  };
  
  let converted = LightDevices::convert_points(point_list.clone(), false);

  let mut controller = match LightDevices::new(i2c_device_id) {
    Ok(v) => v,
    Err(e) => {
      log::error!("Dispatcher failed to get i2c driver: {}", e);
      return;
    },
  };
  
  for (dev_id, dev_adr) in db_devices {
    match converted.iter().position(|(k, _)| k.clone() == dev_id) {
      Some(index) => {
        match controller.set_light_levels(dev_adr as u16, converted[index].1.clone()) {
          Ok(v) => v,
          Err(e) => {
            log::error!("Dispatcher point update failed at set_light_levels for {} : {}", dev_adr, e);
            continue;
          },
        };
      },
      None => (),
    };
  }
}
