mod db;
mod auth;

use std::sync::LazyLock;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use env_logger::Env;
use jsonwebtoken::EncodingKey;
use log::info;
use sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};
use crate::auth::register;

static SALT: LazyLock<[u8; 16]> = LazyLock::new(|| <[u8; 16]>::try_from("1234567812345678".as_bytes()).unwrap());
static SIGN_SECRET: LazyLock<EncodingKey> = LazyLock::new(|| EncodingKey::from_secret("12345678".as_bytes()));

pub struct AppState {
    db: DatabaseConnection
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("INFO"));

    info!("Starting the server...");

    let db: DatabaseConnection = Database::connect("postgres://user:password@localhost/database").await.unwrap();

    info!("Successfully connected to database...");

    Migrator::up(&db, None).await.unwrap();

    let app_state = Data::new(AppState {
        db
    });

    HttpServer::new(
        move || {
            App::new()
                .wrap(Logger::default())
                .app_data(app_state.clone())
                .service(register)
        }
    ).bind("0.0.0.0:8080")?
        .run()
        .await
}
