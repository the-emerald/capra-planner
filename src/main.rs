use actix_web::{App, HttpServer};

pub mod ndl;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(ndl::ndl)
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}