use crate::db::settings::ZHLSettings;
use crate::db::Database;
use actix_web::web::{Data, Json};
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

#[post("/settings/update/zhl")]
pub(crate) async fn update_zhl_settings(
    database: Data<Database>,
    json: Json<ZHLSettings>,
) -> actix_web::Result<HttpResponse> {
    todo!()
}

#[post("/settings/update/general")]
pub(crate) async fn update_general_settings() -> actix_web::Result<HttpResponse> {
    todo!()
}
