use actix_web::{App, HttpServer};
use actix_cors::Cors;

pub mod routes;
pub mod json_repr;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST"])
                    .finish()
            )
            .service(routes::ndl::ndl)
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}