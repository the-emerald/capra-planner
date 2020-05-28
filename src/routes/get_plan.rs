use actix_web::{post, web, HttpResponse};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct GetPlanInput {

}

#[derive(Serialize, Deserialize)]
pub(crate) struct GetPlanOutput {

}

#[post("get_plan")]
pub(crate) async fn route(input: web::Json<GetPlanInput>) -> actix_web::Result<HttpResponse> {
    unimplemented!()
}