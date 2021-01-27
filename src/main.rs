use crate::db::Database;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use std::sync::Arc;

pub mod db;
pub mod json_repr;
pub mod result;
pub mod routes;
pub mod splash;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let bind = format!(
        "{}:{}",
        std::env::var("BIND_ADDRESS").expect("no BIND_ADDRESS set"),
        std::env::var("BIND_PORT").expect("no BIND_PORT set ")
    );

    println!("{}", splash::SPLASH_LOGO);
    println!("Capra Dive Planner Server v{}", env!("CARGO_PKG_VERSION"));
    println!("Starting server on: {}", &bind);

    // Open database
    let database = {
        let db = sled::open(std::env::var("DATABASE").expect("no DATABASE set"))
            .expect("could not open database");
        Arc::new(Database::new(&db).expect("could not initialise database"))
    };

    // Start the server
    HttpServer::new(move || {
        App::new()
            .data(database.clone())
            .wrap(Cors::new().finish())
            .service(routes::user::add_user)
            .service(routes::user::get_user)
            .service(routes::user::get_all_users)
            .service(routes::settings::update_zhl_settings)
            .service(routes::settings::update_general_settings)
            .service(routes::dive::dive_route)
            .service(routes::dive::surface_interval)
            .service(routes::history::dive_history)
    })
    .bind(&bind)?
    .run()
    .await
}
