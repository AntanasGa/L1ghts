use actix_web::{
  web,
  HttpResponse,
  Responder,
};
use diesel::prelude::*;

use crate::{
  types::DbPool,
  models::{
    SinglePoint,
    Points,
  },
  api::{
    ApiError,
    helpers::{
    props::{
      LIGHT_LEVEL_MAX,
      LIGHT_LEVEL_MIN,
    },
    batcher::Batcher,
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
  data: web::Json<SinglePoint>,
  pool: web::Data<DbPool>,
  batcher: web::Data<Batcher>,
) -> Result<impl Responder, ApiError> {

  let point_id = path.into_inner();

  let new_value = if data.value > LIGHT_LEVEL_MAX {
    LIGHT_LEVEL_MAX
  } else if data.value < LIGHT_LEVEL_MIN {
      LIGHT_LEVEL_MIN
  } else {
      data.value
  };

  let mut con = pool.get()
  .map_err(|err| {
    log::error!("Failed to get pool_points: {}", err);
    ApiError::InternalErr
  })?;

  web::block(move || {
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
    Ok(())
  })
  .await
  .map_err(|err| {
    log::error!("Point update block failed: {}", err);
    ApiError::InternalErr
  })??;
    
  batcher.request();
  Ok(HttpResponse::Ok())
}
