#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use actix_cors::Cors;
use diesel::SqliteConnection;
use diesel::r2d2::ConnectionManager;

pub mod routes;
pub mod json_repr;
pub mod db;
pub mod simplified;

type DBPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    // Set up database
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(conn_spec);
    let pool = r2d2::Pool::builder()
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
    })
        .bind(&bind)?
        .run()
        .await
}