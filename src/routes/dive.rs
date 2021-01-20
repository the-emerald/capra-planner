use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

// TODO: Consider moving this to capra crate
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub(crate) enum Algorithm {
    ZHL16,
    VPM,
}

#[post("/dive/")]
pub(crate) async fn dive_route() -> actix_web::Result<HttpResponse> {
    todo!()
}

#[post("/dive/si")]
pub(crate) async fn surface_interval() -> actix_web::Result<HttpResponse> {
    todo!()
}
