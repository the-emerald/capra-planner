use crate::{DBPool};
use actix_web::{post, web, HttpResponse};
use crate::simplified::{SimplifiedZHLSettings};
use crate::db::actions::settings::update_zhl_settings_for_user;
use serde::{Serialize, Deserialize};
use crate::db::actions::user::get_user_by_id;

#[derive(Serialize, Deserialize)]
pub(crate) struct UpdateZHLSettings {
    id: i32,
    new_zhl_settings: SimplifiedZHLSettings
}

#[post("/settings/update/zhl")]
pub(crate) async fn update_zhl_settings(
    pool: web::Data<DBPool>,
    form: web::Json<UpdateZHLSettings>
) -> actix_web::Result<HttpResponse> {
    let conn = pool
        .get()
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    let nz = form.new_zhl_settings.into();
    web::block(move || {
        let user = get_user_by_id(form.id, &conn)?
            .ok_or(diesel::result::Error::NotFound)?;
        update_zhl_settings_for_user(
            &user,
            &nz,
            &conn
        )?;
        Ok::<(), diesel::result::Error>(())
    })
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().finish())
}