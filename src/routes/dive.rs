use crate::db::dives::{Dive, PlanType};
use crate::db::users::UserID;
use crate::db::Database;
use crate::json_repr::dive_segment::JSONDiveSegment;
use crate::json_repr::gas::JSONGas;
use crate::result::ServerDivePlanningError;
use actix_web::web::{Data, Json};
use actix_web::{post, HttpResponse};
use capra::modes::OpenCircuit;
use capra::parameters::DiveParameters;
use capra::DivePlan;
use capra_core::common::{DiveSegment, Gas, SegmentType};
use capra_core::deco::zhl16::tissue_constants::TissueConstants;
use capra_core::deco::zhl16::ZHL16;
use capra_core::deco::{DecoAlgorithm, Tissue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::Arc;
use time::{Duration, OffsetDateTime};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub(crate) enum Algorithm {
    ZHL16,
    VPM,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiveRouteInput {
    id: UserID,
    plan_type: PlanType,
    surface_interval: u64, // Milliseconds
    algorithm: Algorithm,
    parameters: DiveRouteParameters,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiveRouteParameters {
    segments: Vec<(JSONDiveSegment, JSONGas)>,
    deco_gases: Vec<JSONGas>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiveRouteOutput {
    segments: Vec<(JSONDiveSegment, JSONGas)>,
    gas_used: Vec<(JSONGas, usize)>,
}

#[post("/dive/")]
pub(crate) async fn dive_route(
    database: Data<Arc<Database>>,
    json: Json<DiveRouteInput>,
) -> actix_web::Result<HttpResponse> {
    let user = database.users.get_user(&json.id)?.ok_or_else(|| {
        HttpResponse::NotFound()
            .reason("user id does not exist")
            .finish()
    })?;

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

    let si = DiveSegment::new(
        SegmentType::DiveSegment, // not really a good variant to pick
        0,
        0,
        Duration::milliseconds(json.surface_interval as i64),
        general.ascent_rate as isize,
        general.descent_rate as isize,
    )
    .map_err(|_| {
        HttpResponse::InternalServerError()
            .reason("surface interval invalid")
            .finish()
    })?;

    let res = match json.algorithm {
        Algorithm::ZHL16 => {
            let zhl = database.settings.get_zhl_of_user(json.id)?;
            let mut deco = ZHL16::new(
                user.tissue,
                TissueConstants::new_by_variant(zhl.variant),
                zhl.gfl as usize,
                zhl.gfh as usize,
            );

            // Apply SI
            deco.add_dive_segment(
                &si,
                &Gas::new(21, 0, 79).unwrap(),
                10000.0 / general.water_density, // todo: write conversion helper function
            );

            dive(deco, general.into(), &bottom_segments, &deco_gases).await
        }
        Algorithm::VPM => {
            return Ok(HttpResponse::NotImplemented()
                .reason("vpm not implemented")
                .finish());
        }
    };

    // Record into dives
    database.dives.add_dive(&Dive {
        user: json.id,
        plan_type: json.plan_type,
        tissue_before: user.tissue,
        surface_interval: json.surface_interval,
        timestamp: OffsetDateTime::now_utc(),
        zhl_settings: database.settings.get_zhl_of_user(json.id)?,
        general_settings: database.settings.get_general_of_user(json.id)?,
        segments: bottom_segments,
        deco_gases,
    })?;

    // Update tissue if this is an execution
    if json.plan_type == PlanType::Execution {
        database.users.update_tissue(json.id, res.tissue)?;
    }

    Ok(HttpResponse::Ok().json(DiveRouteOutput {
        segments: {
            res.total_segments
                .iter()
                .map(|x| (x.0.into(), x.1.into()))
                .collect()
        },
        gas_used: {
            res.gas_used
                .iter()
                .map(|(k, v)| ((*k).into(), *v))
                .collect()
        },
    }))
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TissueDiveResult {
    tissue: Tissue,
    total_segments: Vec<(DiveSegment, Gas)>,
    gas_used: HashMap<Gas, usize>,
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
) -> TissueDiveResult {
    let result = OpenCircuit::new(deco, deco_gases, bottom_segments, parameters).plan();
    TissueDiveResult {
        tissue: result.deco_algorithm().get_tissue(),
        total_segments: result.total_segments().clone(),
        gas_used: result.gas_used().clone(),
    }
}
