use crate::{DBPool, db};
use actix_web::{post, web, HttpResponse};
use crate::db::models;
use serde::{Serialize, Deserialize};
use actix_web::error::PayloadError::Http2Payload;

#[derive(Serialize, Deserialize)]
struct CombinedUser {
    user: models::user::User,
    tissue: models::tissue::Tissue,
    zhl_settings: models::settings::ZHLSetting,
    vpm_settings: models::settings::VPMSetting,
    gas_plan_settings: models::settings::GasPlanSetting,
}

#[post("/user/new")]
pub(crate) async fn add_user(
    pool: web::Data<DBPool>,
    form: web::Json<models::user::NewUser>
) -> actix_web::Result<HttpResponse> {
    let conn = pool
        .get()
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    let res = web::block(move || {
        let user = db::actions::user::insert_new_user(&form, &conn)?;
        let settings = db::actions::settings::get_all_settings_for_user(&user, &conn)?;
        let tissue = db::actions::tissue::get_tissue_of_user(&user, &conn)?;

        Ok::<CombinedUser, diesel::result::Error>(CombinedUser {
            user,
            tissue,
            zhl_settings: settings.0,
            vpm_settings: settings.1,
            gas_plan_settings: settings.2
        })
    })
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(res))
}