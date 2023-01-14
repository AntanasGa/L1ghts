pub mod models;
pub mod schema;
pub mod db;
pub mod calls;
pub mod api;
pub mod middleware;
pub mod types;
use api::expose_api;
use dotenvy::dotenv;
use types::{
    Tokens,
    SharedStorage,
};
use std::env;
use std::sync::{
    Arc,
    RwLock,
};
use actix_web::{
    HttpServer,
    App,
    Result,
    web,
};
use actix_web::dev::{
    ServiceRequest,
    ServiceResponse,
};
use actix_web::middleware::Logger;
use actix_files as a_fs;
use actix_cors::Cors;
use env_logger::Env;

use diesel::{
    prelude::*,
    r2d2::{
        self,
        ConnectionManager,
    },
};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO: should probably set emergency web mode for when env is missing
    dotenv().ok();
    let host = env::var("HOST").expect("HOST must be set");
    let port = env::var("PORT").expect("PORT must be set").parse::<u16>().expect("PORT must be a number (u16)");
    
    let auth = env::var("JWT_AUTH").expect("JWT_AUTH must be set");
    let refresh = env::var("JWT_REFRESH").expect("JWT_REFRESH must be set");
    let tokens = Tokens {
        auth,
        refresh,
    };

    let i2c_device = env::var("I2C").expect("I2C must be set").parse::<u8>().expect("I2C must be a number (u8)");
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let db_pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Could not initialized database pool");

    let cache_lock = SharedStorage {
        light_update_lock: Arc::new(RwLock::new(false)),
        i2c_device: Arc::new(i2c_device),
    };

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
            App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(tokens.clone()))
            .app_data(web::Data::new(cache_lock.clone()))
            .wrap(
                if env::var("ENV").expect("ENV must be set") == "dev" {
                    Cors::permissive()
                } else {
                    Cors::default()
                }
            )
            .wrap(Logger::default())

            .service(expose_api())
            .service(a_fs::Files::new("/", "public").index_file("index.html").default_handler(index))
    })
        .bind((host, port))?
        .run()
        .await
}


async fn index(reqs: ServiceRequest) -> Result<ServiceResponse> {
    let (req, _) = reqs.into_parts();
    let file = a_fs::NamedFile::open_async("public/index.html").await?;
    let res = file.into_response(&req);
    Ok(ServiceResponse::new(req, res))
}
