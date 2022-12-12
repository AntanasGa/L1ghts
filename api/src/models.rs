use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{ Serialize, Deserialize };


#[derive(Queryable, Debug)]
pub struct Credentials {
    pub id: i32,
    pub user_name: String,
    pub pass: Option<String>,
    pub recovery_key: Option<String>,
    pub recovery_expires: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = credentials)]
pub struct NewCredentials<'a> {
    pub user_name: &'a String,
    pub pass: Option<&'a String>,
    pub recovery_key: Option<&'a String>,
    pub recovery_expires: Option<&'a NaiveDateTime>,
}

#[derive(Queryable, Debug, Serialize, Clone)]
pub struct Devices {
    pub id: i32,
    pub adr: i32,
    pub pairs_of: i32,
    pub endpoint_count: i32,
}

#[derive(Insertable, Debug, Serialize, Clone)]
#[diesel(table_name = devices)]
pub struct NewDevices {
    pub adr: i32,
    pub pairs_of: i32,
    pub endpoint_count: i32,
}

#[derive(Queryable, Debug, Deserialize, Serialize, Clone)]
pub struct Points {
    pub id: i32,
    pub device_id: i32,
    pub val: i32,
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub watts: f32,
    pub active: bool,
    pub tag: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name = points)]
pub struct PointsUpdate {
    pub val: Option<i32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub rotation: Option<f32>,
    pub watts: Option<f32>,
    pub active: Option<bool>,
    pub tag: Option<Option<String>>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = points)]
pub struct NewPoints {
    pub device_id: i32,
    pub val: i32,
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub watts: f32,
    pub active: bool,
    pub tag: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct Presets {
    pub id: i32,
    pub user_id: i32,
    pub preset_name: String,
    pub favorite: bool,
    pub active: bool,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PubPresets {
    pub id: i32,
    pub preset_name: String,
    pub favorite: bool,
    pub icon: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = presets)]
pub struct NewPresets {
    pub user_id: i32,
    pub preset_name: String,
    pub favorite: bool,
    pub active: bool,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PubNewPresets {
    pub preset_name: String,
    pub favorite: bool,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryById {
    pub id: i32,
}

#[derive(Queryable, Debug)]
pub struct PresetItems {
    pub id: i32,
    pub preset_id: i32,
    pub point_id: i32,
    pub val: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = preset_items)]
pub struct NewPresetItems {
    pub preset_id: i32,
    pub point_id: i32,
    pub val: i32,
}

#[derive(Queryable, Debug, Clone)]
pub struct CredentialRefresh {
    pub id: i32,
    pub credential_id: i32,
    pub token: String,
    pub user_agent: String,
    pub created_at: NaiveDateTime,
    pub used_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = credential_refresh)]
pub struct NewCredentialRefresh {
    pub credential_id: i32,
    pub token: String,
    pub user_agent: String,
    pub created_at: NaiveDateTime,
    pub used_at: NaiveDateTime,
}
