use crate::db::settings::{GeneralSettings, ZHLSettings};
use crate::db::users::UserID;
use crate::db::Database;
use actix_web::web::{Data, Json};
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct UpdateZHLSettingsInput {
    new_zhl_settings: ZHLSettings,
    id: UserID,
}

#[post("/settings/update/zhl")]
pub(crate) async fn update_zhl_settings(
    database: Data<Database>,
    json: Json<UpdateZHLSettingsInput>,
) -> actix_web::Result<HttpResponse> {
    if !database.users.get_user(&json.id)?.is_some() {
        return Ok(HttpResponse::NotFound()
            .reason("user id does not exist")
            .finish());
    }
    database
        .settings
        .update_zhl_of_user(json.id, json.new_zhl_settings)?;
    Ok(HttpResponse::Ok().finish())
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct UpdateGeneralSettingsInput {
    new_general_settings: GeneralSettings,
    id: UserID,
}

#[post("/settings/update/general")]
pub(crate) async fn update_general_settings(
    database: Data<Database>,
    json: Json<UpdateGeneralSettingsInput>,
) -> actix_web::Result<HttpResponse> {
    if !database.users.get_user(&json.id)?.is_some() {
        return Ok(HttpResponse::NotFound()
            .reason("user id does not exist")
            .finish());
    }
    database
        .settings
        .update_general_of_user(json.id, json.new_general_settings)?;
    Ok(HttpResponse::Ok().finish())
}
