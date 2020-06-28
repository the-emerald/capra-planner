use crate::{DBPool, db};
use actix_web::{post, web, HttpResponse};
use crate::db::models;

#[post("/user/new")]
pub(crate) async fn add_user(
    pool: web::Data<DBPool>,
    form: web::Json<models::user::NewUser>
) -> actix_web::Result<HttpResponse> {
    let conn = pool
        .get()
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    let user = web::block(move || db::actions::insert_new_user(&form, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().json(user))
}