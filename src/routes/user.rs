use crate::{DBPool, db};
use actix_web::{post, web, HttpResponse};
use crate::db::models;
use serde::{Serialize, Deserialize};
use crate::db::actions::user::get_user_by_id;
use crate::db::models::user::User;

// A simplified user that only contains the id and name field.
#[derive(Serialize, Deserialize)]
struct SimplifiedUser {
    id: i32,
    name: String,
}

impl From<models::user::User> for SimplifiedUser {
    fn from(value: User) -> Self {
        SimplifiedUser {
            id: value.id,
            name: value.name.clone()
        }
    }
}

// A combined struct that contains a user and all their settings.
#[derive(Serialize, Deserialize)]
struct CombinedUser {
    user: SimplifiedUser,
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
            user: user.into(),
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