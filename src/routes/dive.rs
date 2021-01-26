use crate::db::dives::DiveType;
use crate::db::users::UserID;
use crate::db::Database;
use crate::json_repr::dive_segment::JSONDiveSegment;
use crate::json_repr::gas::JSONGas;
use crate::result::ServerDivePlanningError;
use actix_web::web::{Data, Json};
use actix_web::{post, HttpResponse};
use capra::modes::OpenCircuit;
use capra::parameters::DiveParameters;
use capra::{DivePlan, DiveResult};
use capra_core::common::{DiveSegment, Gas};
use capra_core::deco::zhl16::tissue_constants::TissueConstants;
use capra_core::deco::zhl16::ZHL16;
use capra_core::deco::DecoAlgorithm;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub(crate) enum Algorithm {
    ZHL16,
    VPM,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DiveRouteInput {
    id: UserID,
    plan_type: DiveType,
    surface_interval: u64, // Milliseconds
    algorithm: Algorithm,
    parameters: DiveRouteParameters,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DiveRouteParameters {
    segments: Vec<(JSONDiveSegment, JSONGas)>,
    deco_gases: Vec<JSONGas>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DiveRouteOutput {
    segments: Vec<(JSONDiveSegment, JSONGas)>,
    gas_used: Vec<(JSONGas, usize)>,
}

#[post("/dive/")]
pub(crate) async fn dive_route(
    database: Data<Arc<Database>>,
    json: Json<DiveRouteInput>,
) -> actix_web::Result<HttpResponse> {
    let user = database.users.get_user(&json.id)?.ok_or(
        HttpResponse::NotFound()
            .reason("user id does not exist")
            .finish(),
    )?;

    let general = database.settings.get_general_of_user(json.id)?;

    let bottom_segments = json
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

    let res = match json.algorithm {
        Algorithm::ZHL16 => {
            let zhl = database.settings.get_zhl_of_user(json.id)?;
            let deco = ZHL16::new(
                user.tissue,
                TissueConstants::new_by_variant(zhl.variant),
                zhl.gfl as usize,
                zhl.gfh as usize,
            );
            dive(deco, general.into(), &bottom_segments, &deco_gases)
        }
        Algorithm::VPM => {
            return Ok(HttpResponse::NotImplemented()
                .reason("vpm not implemented")
                .finish());
        }
    }
    .await;

    if json.plan_type == DiveType::Execution {
        database
            .users
            .update_tissue(json.id, res.deco_algorithm().tissue())?;
    }

    Ok(HttpResponse::Ok().json(DiveRouteOutput {
        segments: {
            res.total_segments()
                .iter()
                .map(|x| (x.0.into(), x.1.into()))
                .collect()
        },
        gas_used: {
            res.gas_used()
                .iter()
                .map(|(k, v)| ((*k).into(), *v))
                .collect()
        },
    }))
}

#[post("/dive/si")]
pub(crate) async fn surface_interval() -> actix_web::Result<HttpResponse> {
    todo!()
}

async fn dive<T: DecoAlgorithm>(
    deco: T,
    parameters: DiveParameters,
    bottom_segments: &[(DiveSegment, Gas)],
    deco_gases: &[(Gas, Option<usize>)],
) -> DiveResult<T> {
    let open_circuit = OpenCircuit::new(deco, deco_gases, bottom_segments, parameters);
    open_circuit.plan()
}
