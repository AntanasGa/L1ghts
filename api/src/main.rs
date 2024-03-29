pub mod models;
pub mod schema;
pub mod db;
pub mod calls;
pub mod api;
pub mod middleware;
pub mod types;
pub mod dispatcher;

use api::expose_api;
use api::helpers::batcher::Batcher;
use api::helpers::i2c::LightDevices;
use dotenvy::dotenv;
use types::{
    Tokens,
    SharedStorage,
};
use std::env;
use std::sync::Arc;
use std::time::Duration;
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

    let setup_secret = env::var("SETUP_SECRET").expect("SETUP_SECRET must be set");

    let i2c_device = env::var("I2C").expect("I2C must be set").parse::<u8>().expect("I2C must be a number (u8)");

    match LightDevices::test(i2c_device).err() {
        Some(err) => {
            panic!("{}", err);
        },
        None => {}
    }
    
    let default_rate_ms = 200;
    let dispatcher_rate_ms = match env::var("DISPATCHER_RATE_MS") {
        Ok(v) => match v.parse::<u64>() {
            Ok(iv) => iv,
            Err(_) => default_rate_ms.to_owned(),
        },
        Err(_) => default_rate_ms.to_owned(),
    };
    
    // Duration::from_millis();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let db_pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Could not initialized database pool");

    let cache_lock = SharedStorage {
        i2c_device: Arc::new(i2c_device),
        setup_secret: Arc::new(setup_secret),
    };

    let batcher = Batcher::new();

    let background_batcher = batcher.clone();
    let db_pool_batcher = db_pool.clone();
    let device_id_batcher = i2c_device.clone();
    actix_web::rt::spawn(async move {
        loop {
            actix_web::rt::time::sleep(Duration::from_millis(dispatcher_rate_ms)).await;

            if background_batcher.pull() {
                dispatcher::dispatch(db_pool_batcher.clone(), device_id_batcher).await;
            }
        }
    });

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
            App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(tokens.clone()))
            .app_data(web::Data::new(cache_lock.clone()))
            .app_data(web::Data::new(batcher.clone()))
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
