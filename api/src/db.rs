use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::ConnectionError;
use std::{ env::VarError, env };


pub fn new() -> Result<PgConnection, DbError> {
    let database_url = env::var("DATABASE_URL")?;
    let db = PgConnection::establish(&database_url)?;
    Ok(db)
}

#[derive(Debug)]
pub enum DbError {
    ConnectionError(ConnectionError),
    VarError(VarError),
}


impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::ConnectionError(ce) => write!(f, "{}", ce),
            DbError::VarError(ve) => write!(f, "{}", ve),
        }
    }
}

impl std::error::Error for DbError {}

impl From<ConnectionError> for DbError {
    fn from(e: ConnectionError) -> Self {
        DbError::ConnectionError(e)
    }
}

impl From<VarError> for DbError {
    fn from(e: VarError) -> Self {
        DbError::VarError(e)
    }
}
