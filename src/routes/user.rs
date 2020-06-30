use crate::{DBPool, db};
use actix_web::{post, web, HttpResponse};
use crate::db::models;
use serde::{Serialize, Deserialize};
use crate::db::actions::user::get_user_by_id;
use crate::simplified::{SimplifiedUser, SimplifiedTissue, SimplifiedZHLSettings, SimplifiedVPMSettings, SimplifiedGeneralSettings};

// A combined struct that contains a user and all their settings.
#[derive(Serialize, Deserialize)]
struct CombinedUser {
    user: SimplifiedUser,
    tissue: SimplifiedTissue,
    zhl_settings: SimplifiedZHLSettings,
    vpm_settings: SimplifiedVPMSettings,
    gas_plan_settings: SimplifiedGeneralSettings,
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
            user: user.into(),
            tissue: tissue.into(),
            zhl_settings: settings.0.into(),
            vpm_settings: settings.1.into(),
            gas_plan_settings: settings.2.into()
        })
    })
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(res))
}

#[derive(Serialize, Deserialize)]
pub struct UserID {
    pub id: i32
}

#[post("/user")]
pub(crate) async fn get_user(
    pool: web::Data<DBPool>,
    form: web::Json<UserID>
) -> actix_web::Result<HttpResponse> {
    let conn = pool
        .get()
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    let res = web::block(move || {
        let user = get_user_by_id(form.id, &conn)?
            .ok_or(diesel::result::Error::NotFound)?;
        let settings = db::actions::settings::get_all_settings_for_user(&user, &conn)?;
        let tissue = db::actions::tissue::get_tissue_of_user(&user, &conn)?;

        Ok::<CombinedUser, diesel::result::Error>(CombinedUser {
            user: user.into(),
            tissue: tissue.into(),
            zhl_settings: settings.0.into(),
            vpm_settings: settings.1.into(),
            gas_plan_settings: settings.2.into()
        })
    })
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(res))
}

#[post("/user/all")]
pub(crate) async fn get_all_users(
    pool: web::Data<DBPool>
) -> actix_web::Result<HttpResponse> {
    let conn = pool
        .get()
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    let res = web::block(move || {
        db::actions::user::get_list_all_users(&conn)
    })
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<SimplifiedUser>>();

    Ok(HttpResponse::Ok().json(res))
}