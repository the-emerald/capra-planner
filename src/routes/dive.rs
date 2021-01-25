use crate::db::users::UserID;
use crate::db::Database;
use crate::json_repr::dive_segment::JSONDiveSegment;
use crate::json_repr::gas::JSONGas;
use crate::result::ServerDivePlanningError;
use actix_web::web::{Data, Json};
use actix_web::{post, web, HttpResponse};
use capra::parameters::DiveParameters;
use capra::{DivePlan, DiveResult};
use capra_core::common::{DiveSegment, Gas};
use capra_core::deco::zhl16::tissue_constants::TissueConstants;
use capra_core::deco::zhl16::{Variant, ZHL16};
use capra_core::deco::DecoAlgorithm;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub(crate) enum Algorithm {
    ZHL16,
    VPM,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum PlanType {
    #[serde(alias = "PLAN")]
    Plan,
    #[serde(alias = "EXECUTE")]
    Execute,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DiveRouteInput {
    id: UserID,
    plan_type: PlanType,
    surface_interval: u64, // Milliseconds
    algorithm: Algorithm,
    parameters: DiveRouteParameters,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DiveRouteParameters {
    segments: Vec<(JSONDiveSegment, JSONGas)>,
    deco_gases: Vec<JSONGas>,
}

#[post("/dive/")]
pub(crate) async fn dive_route(
    database: Data<Database>,
    json: Json<DiveRouteInput>,
) -> actix_web::Result<HttpResponse> {
    let user = database.users.get_user(&json.id)?.ok_or(
        HttpResponse::NotFound()
            .reason("user id does not exist")
            .finish(),
    )?;

    let general = database.settings.get_general_of_user(json.id)?;

    let segments = json
        .parameters
        .segments
        .iter()
        .map(|x| Ok((x.0.try_into()?, x.1.try_into()?)))
        .collect::<Result<Vec<(DiveSegment, Gas)>, ServerDivePlanningError>>()?;

    let deco_gases = json
        .parameters
        .deco_gases
        .iter()
        .cloned()
        .map(|x| Ok((x.try_into()?, x.max_op_depth())))
        .collect::<Result<Vec<(Gas, Option<usize>)>, ServerDivePlanningError>>()?;

    match json.algorithm {
        Algorithm::ZHL16 => {
            let zhl = database.settings.get_zhl_of_user(json.id)?;
            let deco = ZHL16::new(
                user.tissue,
                TissueConstants::new_by_variant(zhl.variant),
                zhl.gfl as usize,
                zhl.gfh as usize,
            );
        }
        Algorithm::VPM => {
            return Ok(HttpResponse::NotImplemented()
                .reason("vpm not implemented")
                .finish());
        }
    }

    Ok(HttpResponse::Ok().finish())
}

#[post("/dive/si")]
pub(crate) async fn surface_interval() -> actix_web::Result<HttpResponse> {
    todo!()
}

async fn dive<T: DecoAlgorithm>(deco: T, parameters: DiveParameters) -> DiveResult<T> {
    unimplemented!()
}
