use diesel::{
    insert_into,
    RunQueryDsl,
    delete,
    prelude::*,
};
use crate::api::ApiError;
use crate::models::{
    NewPoints,
    Points,
};
use crate::schema::points::dsl::*;
use crate::types::DbCon;

pub fn fill_diff(con: &mut DbCon, diff: usize, devc_id: i32, fill_start: i32) -> Result<usize, ApiError> {
    let mut fill_location = fill_start.clone();
    let insert_points: _ = (0..diff).into_iter()
    .map(|_| {
        fill_location += 1;
        NewPoints {
            device_id: devc_id.clone(),
            device_position: fill_location,
            height: 1.0,
            width: 1.0,
            rotation: 0.0,
            watts: 0.0,
            x: 0.0,
            y: 0.0,
            val: 0,
            active: false,
            tag: None,
        }
    })
    .collect::<Vec<NewPoints>>();
    insert_into(points).values(&insert_points).execute(con)
    .map_err(|err| {
        log::error!("Failed inserting device points: {}", err);
        ApiError::InternalErr
    })
}

/// deleting items from the last `device_position`
pub fn reduce_diff(con: &mut DbCon, diff: usize, point_list: Vec<Points>) -> Result<(), ApiError> {
    if point_list.len() == 0 {
        return Ok(());
    }
    if point_list.len() < diff {
        return Err(ApiError::InternalErr);
    }

    let mut sorted_points = point_list.clone();
    sorted_points.sort_by_key(|point| point.device_position);

    let mut delete_ = delete(points).into_boxed();
    delete_ = delete_.filter(id.eq(sorted_points[sorted_points.len() - 1].id));
    if diff > 1 {
        for i in 2..=diff {
            delete_ = delete_.or_filter(id.eq(sorted_points[sorted_points.len() - i].id));
        }
    }

    delete_.execute(con)
    .map_err(|err| {
        log::error!("Failed inserting device points: {}", err);
        ApiError::InternalErr
    })?;
    Ok(())
}