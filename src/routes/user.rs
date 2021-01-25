use crate::db::users::{User, UserID};
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
    let user_id = database.users.add_user(json.clone().name)?.ok_or(
        HttpResponse::Conflict()
            .reason("user already exists")
            .finish(),
    )?;

    // Set up settings
    database.settings.initialise_user(user_id)?;

    Ok(HttpResponse::Ok().finish())
}
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct GetUserData {
    id: UserID,
}

#[post("/user")]
pub(crate) async fn get_user(
    database: Data<Database>,
    json: Json<GetUserData>,
) -> actix_web::Result<HttpResponse> {
    let user = database
        .users
        .get_user(&json.id)?
        .ok_or(HttpResponse::Conflict().reason("user not found").finish())?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("/user/all")]
pub(crate) async fn get_all_users() -> actix_web::Result<HttpResponse> {
    todo!()
}
