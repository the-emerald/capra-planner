use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

#[post("/settings/update/zhl")]
pub(crate) async fn update_zhl_settings() -> actix_web::Result<HttpResponse> {
    todo!()
}

#[post("/settings/update/general")]
pub(crate) async fn update_general_settings() -> actix_web::Result<HttpResponse> {
    todo!()
}
