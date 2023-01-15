use std::sync::{
    Arc,
    RwLock
};

use crate::calls::AuthToken;
use jsonwebtoken::TokenData;

use diesel::{
    prelude::*,
    r2d2::{
        self,
        ConnectionManager,
        PooledConnection,
    },
};

#[derive(Clone)]
pub struct Tokens {
    pub auth: String,
    pub refresh: String,
}

#[derive(Debug, Clone)]
pub struct SharedStorage {
    pub light_update_lock: Arc<RwLock<bool>>,
    pub i2c_device: Arc<u8>,
    pub setup_secret: Arc<String>,
}

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub type JWTAuthToken = TokenData<AuthToken>;

pub type DbCon = PooledConnection<ConnectionManager<PgConnection>>;
