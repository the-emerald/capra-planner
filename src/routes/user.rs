use crate::{DBPool, db};
use actix_web::{post, web, HttpResponse};

#[post("/user/new")]
pub(crate) async fn add_user(
    pool: web::Data<DBPool>,
    form: web::Json<db::models::user::NewUser>
) -> actix_web::Result<HttpResponse> {
    unimplemented!()
}