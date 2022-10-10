#[macro_use]
extern crate diesel;

use std::time::Duration;

use actix_identity::IdentityMiddleware;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod error;
mod models;
mod routes;
mod schema;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var(
        "RUST_LOG",
        "simple-auth-server=debug,actix_web=info,actix_server=info",
    );
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    let secret_key = Key::generate();
    let redis_store = RedisSessionStore::new(
        std::env::var("REDIS_URL")
            .expect("REDIS_URL must be set")
            .as_str(),
    )
    .await
    .expect("Failed to connect to redis");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(
                IdentityMiddleware::builder()
                    .login_deadline(Some(Duration::from_secs(21600)))
                    .visit_deadline(Some(Duration::from_secs(1200)))
                    .build(),
            )
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .app_data(web::JsonConfig::default().limit(4096))
            .service(
                web::scope("/api/v1")
                    .service(
                        web::resource("/register")
                            .route(web::post().to(routes::register::generate_invitation))
                    )
                    .service(
                        web::resource("/register/{invitation_id}")
                            //.route(web::get().to(routes::auth::check_invitation))
                            .route(web::post().to(routes::register::register_user))
                    )
                    .service(
                        web::resource("/auth")
                            .route(web::post().to(routes::auth::login))
                            .route(web::delete().to(routes::auth::logout))
                            .route(web::get().to(routes::auth::get_me))
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
