use crate::db::settings::{GeneralSettings, ZHLSettings};
use crate::db::users::{User, UserID};
use crate::db::Database;
use actix_web::web::{Data, Json};
use actix_web::{post, HttpResponse};
use capra_core::deco::Tissue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct AddUserInput {
    name: String,
}

#[post("/user/new")]
pub(crate) async fn add_user(
    database: Data<Database>,
    json: Json<AddUserInput>,
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
pub struct GetUserInput {
    id: UserID,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetUserOutput {
    user: UserWithId,
    zhl_settings: ZHLSettings,
    vpm_settings: (), // TODO: VPM not actually implemented yet
    general_settings: GeneralSettings,
    tissue: Tissue,
}

#[post("/user")]
pub(crate) async fn get_user(
    database: Data<Database>,
    json: Json<GetUserInput>,
) -> actix_web::Result<HttpResponse> {
    let user = database
        .users
        .get_user(&json.id)?
        .ok_or(HttpResponse::Conflict().reason("user not found").finish())?;

    Ok(HttpResponse::Ok().json(GetUserOutput {
        user: UserWithId {
            id: json.id,
            name: user.name,
        },
        zhl_settings: database.settings.get_zhl_of_user(json.id)?,
        vpm_settings: (),
        general_settings: database.settings.get_general_of_user(json.id)?,
        tissue: user.tissue,
    }))
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserWithId {
    id: UserID,
    name: String,
}

#[post("/user/all")]
pub(crate) async fn get_all_users(database: Data<Database>) -> actix_web::Result<HttpResponse> {
    let all = database
        .users
        .get_all_users()?
        .into_iter()
        .map(|x| UserWithId {
            id: x.0,
            name: x.1.name,
        })
        .collect::<Vec<UserWithId>>();

    Ok(HttpResponse::Ok().json(all))
}
