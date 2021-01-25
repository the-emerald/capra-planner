use crate::db::users::User;
use crate::db::Database;
use actix_web::web::{Data, Json};
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[derive(Clone, Serialize, Deserialize)]
pub struct AddUserData {
    name: String,
}

#[post("/user/new")]
pub(crate) async fn add_user(
    database: Data<Database>,
    json: Json<AddUserData>,
) -> actix_web::Result<HttpResponse> {
    // Set up new user
    let new_user = database.users.add_user(json.clone().name)?.ok_or(
        HttpResponse::Conflict()
            .reason("user already exists")
            .finish(),
    )?;

    // Set up settings
    database.settings.initialise_user(new_user.id)?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/user")]
pub(crate) async fn get_user() -> actix_web::Result<HttpResponse> {
    todo!()
}

#[post("/user/all")]
pub(crate) async fn get_all_users() -> actix_web::Result<HttpResponse> {
    todo!()
}
