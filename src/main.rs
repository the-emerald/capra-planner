#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use actix_cors::Cors;
use diesel::{SqliteConnection};
use diesel::r2d2::ConnectionManager;
use crate::db::connection_options::ConnectionOptions;
use std::time::Duration;

pub mod routes;
pub mod json_repr;
pub mod db;
pub mod simplified;
pub mod result;

type DBPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    // Set up database
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
    let pool = r2d2::Pool::builder()
        .connection_customizer(Box::new(ConnectionOptions::default()))
        .max_size(1)
        .connection_timeout(Duration::from_secs(5))
        .build(manager)
        .expect("failed to create pool");

    let bind = format!("{}:{}",
        std::env::var("BIND_ADDRESS").expect("BIND_ADDRESS"),
        std::env::var("BIND_PORT").expect("BIND_PORT")
    );

    println!("Starting server on: {}", &bind);

    // Start the server
    HttpServer::new(move || {
        App::new()
            .data(
                pool.clone()
            )
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST"])
                    .finish()
            )
            .service(routes::user::add_user)
            .service(routes::user::get_user)
            .service(routes::user::get_all_users)

            .service(routes::settings::update_zhl_settings)
            .service(routes::settings::update_general_settings)

            .service(routes::dive::dive_route)
    })
        .bind(&bind)?
        .run()
        .await
}