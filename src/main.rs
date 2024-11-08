mod db;
mod auth;

use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};

pub struct AppState {
    db: DatabaseConnection
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: DatabaseConnection = Database::connect("postgres://user:password@localhost/database").await.unwrap();

    Migrator::up(&db, None).await.unwrap();

    HttpServer::new(
        move || {
            App::new().wrap(
                Logger::default()
            )
        }
    ).bind("0.0.0.0:8080")?
        .run()
        .await
}
