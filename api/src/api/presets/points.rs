use actix_web::web;
use diesel::{
    prelude::*,
    update,
};

use crate::{
    types::DbPool,
    middleware::auth::TokenData,
    models::{
        QueryById,
        PresetItems,
        Points,
    },
    api::ApiError,
};

pub async fn put(
    pool: web::Data<DbPool>,
    token: web::ReqData<TokenData>,
    data: web::Json<QueryById>
) -> Result<web::Json<QueryById>, ApiError> {
    let uid = token.claims.uid;
    let preset_id = web::block(move || {
        let mut con = pool.get()
        .map_err(|err| {
            log::error!("Failed to get pool: {}", err);
            ApiError::InternalErr
        })?;

        use crate::schema::presets::dsl::*;
        let user_preset_count = presets.filter(
            crate::schema::presets::dsl::id.eq(data.id.clone())
            .and(crate::schema::presets::dsl::user_id.eq(uid.clone()))
        )
        .count()
        .get_result::<i64>(&mut con)
        .map_err(|err| {
            log::error!("Failed to fetch editable preset: {}", err);
            ApiError::InternalErr
        })?;
        if user_preset_count > 1 {
            return Err(ApiError::InternalErr);
        }
        if user_preset_count < 1 {
            return Err(ApiError::Conflict);
        }

        use crate::schema::preset_items::dsl::*;
        let selected_preset_items = preset_items.filter(
            crate::schema::preset_items::dsl::preset_id.eq(data.id.clone())
        )
        .load::<PresetItems>(&mut con)
        .map_err(|err| {
            log::error!("Failed to fetch current update preset items: {}", err);
            ApiError::InternalErr
        })?;

        use crate::schema::points::dsl::*;
        let current_values = points.load::<Points>(&mut con)
        .map_err(|err| {
            log::error!("Failed to fetch current points: {}", err);
            ApiError::InternalErr
        })?;

        for point in selected_preset_items {
            match current_values.iter().position(|v| v.id == point.point_id) {
                None => continue,
                Some(selector) => {
                    
                    update(preset_items).filter(
                        crate::schema::preset_items::dsl::id.eq(point.id.clone())
                    )
                    .set(
                        crate::schema::preset_items::dsl::val.eq(current_values[selector].val.clone())
                    )
                    .execute(&mut con)
                    .map_err(|err| {
                        log::error!("Failed to update preset point [{}]: {}", point.id, err);
                        ApiError::InternalErr
                    })?;
                } 
            }
        }
        Ok(data.id.clone())
    })
    .await
    .map_err(|err| {
        log::error!("Preset fetching block failed: {}", err);
        ApiError::InternalErr
    })??;
    // see nothing wrong by echoing the request
    let result = QueryById { id: preset_id };
    Ok(web::Json(result))
}
