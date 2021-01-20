use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[post("/user/new")]
pub(crate) async fn add_user() -> actix_web::Result<HttpResponse> {
    todo!()
}

#[post("/user")]
pub(crate) async fn get_user() -> actix_web::Result<HttpResponse> {
    todo!()
}

#[post("/user/all")]
pub(crate) async fn get_all_users() -> actix_web::Result<HttpResponse> {
    todo!()
}
