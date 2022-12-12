use diesel::{
    insert_into,
    RunQueryDsl,
    delete,
    prelude::*,
};
use crate::api::ApiError;
use crate::models::NewPoints;
use crate::schema::points::dsl::*;
use crate::types::DbCon;

pub fn fill_diff(con: &mut DbCon, diff: usize, devc_id: i32) -> Result<usize, ApiError> {
    let insert_points: _ = (0..diff).into_iter()
    .map(|_| NewPoints {
            device_id: devc_id.clone(),
            height: 1.0,
            width: 1.0,
            rotation: 0.0,
            watts: 0.0,
            x: 0.0,
            y: 0.0,
            val: 0,
            active: false,
            tag: None,
    })
    .collect::<Vec<NewPoints>>();
    insert_into(points).values(&insert_points).execute(con)
    .map_err(|err| {
        log::error!("Failed inserting device points: {}", err);
        ApiError::InternalErr
    })
}

pub fn reduce_diff(con: &mut DbCon, diff: usize, id_list: Vec<i32>) -> Result<(), ApiError> {
    if id_list.len() == 0 {
        return Ok(());
    }
    if diff > id_list.len() {
        return Err(ApiError::InternalErr);
    }
    let mut delete_ = delete(points).into_boxed();
    delete_ = delete_.filter(id.eq(id_list[id_list.len() - 1]));
    if diff > 1 {
        for i in 2..=diff {
            delete_ = delete_.or_filter(id.eq(id_list[id_list.len() - i]));
        }
    }
    delete_.execute(con)
    .map_err(|err| {
        log::error!("Failed inserting device points: {}", err);
        ApiError::InternalErr
    })?;
    Ok(())
}