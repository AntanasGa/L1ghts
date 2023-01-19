pub mod active;
pub mod points;

use std::vec;
use crate::{
    api::ApiError,
    types::DbPool,
    models::{
        PubNewPresets,
        PubPresets,
        Presets,
        NewPresets,
        Points,
        NewPresetItems,
        QueryById,
    },
    middleware::auth::TokenData,
};
use actix_web::{
    web,
};
use diesel::{
    prelude::*,
    insert_into,
    update,
    delete,
};

pub async fn get(pool: web::Data<DbPool>, token: web::ReqData<TokenData>) -> Result<web::Json<Vec<PubPresets>>, ApiError> {
    let mut con = pool.get()
    .map_err(|err| {
        log::error!("Failed to get pool: {}", err);
        ApiError::InternalErr
    })?;
    let uid = token.claims.uid.clone();
    let user_presets = web::block(move || {
        use crate::schema::presets::dsl::*;
        presets.filter(user_id.eq(uid))
        .load::<Presets>(&mut con)
        .map_err(|err| {
            log::error!("Fetching presets failed: {}", err);
            ApiError::InternalErr
        })
    })
    .await
    .map_err(|err| {
        log::error!("Preset fetching block failed: {}", err);
        ApiError::InternalErr
    })??;
    let mut send_presets: Vec<PubPresets> = vec![];
    for preset in &user_presets {
        send_presets.push(PubPresets {
            id: preset.id,
            preset_name: preset.preset_name.clone(),
            favorite: preset.favorite,
            icon: preset.icon.clone(),
        });
    }
    Ok(web::Json(send_presets))
}

pub async fn post(
    pool: web::Data<DbPool>,
    token: web::ReqData<TokenData>,
    data: web::Json<PubNewPresets>,
) -> Result<web::Json<Vec<PubPresets>>, ApiError> {

    let uid = token.claims.uid.clone();

    use crate::schema::presets::dsl::*;
    use crate::schema::points::dsl::*;
    use crate::schema::preset_items::dsl::*;

    // validate that preset with same name exists
    let current_comp_pool = pool.clone();
    let active_points = web::block(move || {
        let mut con = current_comp_pool.get()
        .map_err(|err| {
            log::error!("Failed to get current_comp_pool: {}", err);
            ApiError::InternalErr
        })?;
        let current_points = points.load::<Points>(&mut con)
        .map_err(|err| {
            log::error!("Failed to fetch all points: {}", err);
            ApiError::InternalErr
        })?;
        let mut select_preset_points_ = preset_items.into_boxed();
        let mut filter_first_ = true;
        for point in &current_points {
            // this generates clause tree ([(...](expression)[ OR expression...)])
            // but it is 1 request instead of one for each
            select_preset_points_ = match filter_first_ {
                true => {
                    filter_first_ = false;
                    select_preset_points_.filter(
                        point_id.eq(point.id.clone())
                        .and(crate::schema::preset_items::dsl::val.eq(point.val.clone()))
                    )
                },
                false => {
                    select_preset_points_.or_filter(
                        point_id.eq(point.id.clone())
                        .and(crate::schema::preset_items::dsl::val.eq(point.val.clone()))
                    )
                },
            }
        }
        let matches_on_presets = select_preset_points_.count().get_result::<i64>(&mut con)
        .map_err(|err| {
            log::error!("Failed to fetch preset item list: {}", err);
            ApiError::InternalErr
        })?;
        if matches_on_presets as usize == current_points.len() {
            return Err(ApiError::Conflict);
        }
        Ok(current_points)
    })
    .await
    .map_err(|err| {
        log::error!("Preset comparison block failed: {}", err);
        ApiError::InternalErr
    })??;

    // creating new template
    let insert_pool = pool.clone();
    web::block(move || {
        let mut con = insert_pool.get()
        .map_err(|err| {
            log::error!("Failed to get insert_pool: {}", err);
            ApiError::InternalErr
        })?;

        // seting other presets to false, just in case
        update(presets).filter(crate::schema::presets::active.eq(true))
        .set(crate::schema::presets::active.eq(false))
        .execute(&mut con)
        .map_err(|err| {
            log::error!("Failed to update presets [pre insert]: {}", err);
            ApiError::InternalErr
        })?;

        insert_into(presets).values(NewPresets {
            user_id: uid.clone(),
            preset_name: data.preset_name.clone(),
            favorite: data.favorite,
            // FIXME: missing icon upload route
            icon: data.icon.clone(),
            active: true,
        })
        .execute(&mut con)
        .map_err(|err| {
            log::error!("Failed to insert new preset: {}", err);
            ApiError::InternalErr
        })?;

        let inserted_preset_list = presets.order(crate::schema::presets::id.desc())
        .limit(1)
        .load::<Presets>(&mut con)
        .map_err(|err| {
            log::error!("Failed to fetch new preset: {}", err);
            ApiError::InternalErr
        })?;

        let inserted_preset = inserted_preset_list[0].clone();

        let new_preset_items: _ = active_points.iter()
        .map(move |v| NewPresetItems {point_id: v.id, preset_id: inserted_preset.id, val: v.val })
        .collect::<Vec<NewPresetItems>>();

        insert_into(preset_items).values(new_preset_items)
        .execute(&mut con)
        .map_err(|err| {
            log::error!("Failed to insert new preset items: {}", err);
            ApiError::InternalErr
        })?;
        Ok(())
    })
    .await
    .map_err(|err| {
        log::error!("Preset inserting block failed: {}", err);
        ApiError::InternalErr
    })??;

    let fetch_all_pool = pool.clone();
    let user_presets = web::block(move || {
        let mut con = fetch_all_pool.get()
        .map_err(|err| {
            log::error!("Failed to get fetch_all_pool: {}", err);
            ApiError::InternalErr
        })?;
        presets.filter(user_id.eq(uid))
        .load::<Presets>(&mut con)
        .map_err(|err| {
            log::error!("Fetching presets failed: {}", err);
            ApiError::InternalErr
        })
    })
    .await
    .map_err(|err| {
        log::error!("Preset fetching block failed: {}", err);
        ApiError::InternalErr
    })??;
    let mut send_presets: Vec<PubPresets> = vec![];
    for preset in &user_presets {
        send_presets.push(PubPresets {
            id: preset.id,
            preset_name: preset.preset_name.clone(),
            favorite: preset.favorite,
            icon: preset.icon.clone(),
        });
    }
    Ok(web::Json(send_presets))
}

pub async fn upd(
    pool: web::Data<DbPool>,
    token: web::ReqData<TokenData>,
    data: web::Json<PubPresets>,
) -> Result<web::Json<Vec<PubPresets>>, ApiError> {
    let uid = token.claims.uid.clone();

    let response = web::block(move || {
        let mut con = pool.get()
        .map_err(|err| {
            log::error!("Failed to get pool: {}", err);
            ApiError::InternalErr
        })?;
        use crate::schema::presets::dsl::*;
        let update_item_count: i64 = presets.filter(
            id.eq(data.id.clone()).and(user_id.eq(uid.clone()))
        ).count()
        .get_result::<i64>(&mut con)
        .map_err(|err| {
            log::error!("Failed to do preflight check for preset update: {}", err);
            ApiError::InternalErr
        })?;
        if update_item_count > 1 {
            log::error!("Prieset unique ids have failed");
            return Err(ApiError::InternalErr);
        }
        if update_item_count < 1 {
            return Err(ApiError::Conflict);
        }
        update(presets)
        .filter(
            id.eq(data.id)
            .and(user_id.eq(uid.clone()))
        )
        .set((
            preset_name.eq(data.preset_name.clone()),
            favorite.eq(data.favorite),
            icon.eq(data.icon.clone()),
        ))
        .execute(&mut con)
        .map_err(|err| {
            log::error!("Failed to update preset id [{}]: {},", data.id.clone(), err);
            ApiError::InternalErr
        })?;
        let user_presets = presets.filter(user_id.eq(uid))
        .load::<Presets>(&mut con)
        .map_err(|err| {
            log::error!("Fetching presets failed: {}", err);
            ApiError::InternalErr
        })?;
        let mut send_presets: Vec<PubPresets> = vec![];
        for preset in &user_presets {
            send_presets.push(PubPresets {
                id: preset.id,
                preset_name: preset.preset_name.clone(),
                favorite: preset.favorite,
                icon: preset.icon.clone(),
            });
        }
        Ok(send_presets)
    })
    .await
    .map_err(|err| {
        log::error!("Preset update block failed: {}", err);
        ApiError::InternalErr
    })??;

    Ok(web::Json(response))
}

pub async fn del(
    pool: web::Data<DbPool>,
    token: web::ReqData<TokenData>,
    data: web::Json<QueryById>,
) -> Result<web::Json<Vec<PubPresets>>, ApiError> {
    let uid = token.claims.uid.clone();
    let presets = web::block(move || {
        let mut con = pool.get()
        .map_err(|err| {
            log::error!("Failed to get pool: {}", err);
            ApiError::InternalErr
        })?;
        // this should also update items in preset_items due to cascade
        use crate::schema::presets::dsl::*;
        let delete_count = delete(
            presets.filter(id.eq(data.id.clone())
            .and(user_id.eq(uid.clone())))
        )
        .execute(&mut con)
        .map_err(|err| {
            log::error!("Failed to delete preset: {}", err);
            ApiError::InternalErr
        })?;

        if delete_count > 1 {
            log::error!("Prieset unique ids have failed");
            return Err(ApiError::InternalErr);
        }
        if delete_count < 1 {
            return Err(ApiError::Conflict);
        }
        let user_presets = presets.filter(user_id.eq(uid))
        .load::<Presets>(&mut con)
        .map_err(|err| {
            log::error!("Fetching presets failed: {}", err);
            ApiError::InternalErr
        })?;
        let mut send_presets: Vec<PubPresets> = vec![];
        for preset in &user_presets {
            send_presets.push(PubPresets {
                id: preset.id,
                preset_name: preset.preset_name.clone(),
                favorite: preset.favorite,
                icon: preset.icon.clone(),
            });
        }
        Ok(send_presets)
    })
    .await
    .map_err(|err| {
        log::error!("Preset deleting block failed: {}", err);
        ApiError::InternalErr
    })??;
    Ok(web::Json(presets))
}
