use actix_cors::Cors;
use actix_web::{App, HttpServer};
use std::time::Duration;

pub mod json_repr;
pub mod result;
pub mod routes;
pub mod splash;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let bind = format!(
        "{}:{}",
        std::env::var("BIND_ADDRESS").expect("BIND_ADDRESS"),
        std::env::var("BIND_PORT").expect("BIND_PORT")
    );

    println!("{}", splash::SPLASH_LOGO);
    println!("Capra Dive Planner Server v{}", env!("CARGO_PKG_VERSION"));
    println!("Starting server on: {}", &bind);

    // Start the server
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::new().finish())
            .service(routes::user::add_user)
            .service(routes::user::get_user)
            .service(routes::user::get_all_users)
            .service(routes::settings::update_zhl_settings)
            .service(routes::settings::update_general_settings)
            .service(routes::dive::dive_route)
            .service(routes::dive::surface_interval)
    })
    .bind(&bind)?
    .run()
    .await
}
