use crate::db::dives::{Dive};
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
use time::{OffsetDateTime, Duration};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub(crate) enum Algorithm {
    ZHL16,
    VPM,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiveRouteInput {
    id: UserID,
    algorithm: Algorithm,
    parameters: DiveRouteParameters,
    start_tissue: Option<Tissue>,
    record: bool
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
    end_tissue: Tissue,
}

#[post("/dive/")]
pub(crate) async fn plan(
    database: Data<Arc<Database>>,
    json: Json<DiveRouteInput>,
) -> actix_web::Result<HttpResponse> {
    dive(&database, &json).await
}

async fn dive(database: &Data<Arc<Database>>, json: &Json<DiveRouteInput>) -> actix_web::Result<HttpResponse> {
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

    let tissue = match json.start_tissue {
        Some(t) => { t }
        None => { user.tissue }
    };

    let res = match json.algorithm {
        Algorithm::ZHL16 => {
            let zhl = database.settings.get_zhl_of_user(json.id)?;
            let deco = ZHL16::new(
                tissue,
                TissueConstants::new_by_variant(zhl.variant),
                zhl.gfl as usize,
                zhl.gfh as usize,
            );

            get_dive_result(deco, general.into(), &bottom_segments, &deco_gases).await
        }
        Algorithm::VPM => {
            return Ok(HttpResponse::NotImplemented()
                .reason("vpm not implemented")
                .finish());
        }
    };

    // Record into dives
    if json.record {
        database.dives.add_dive(&Dive {
            user: json.id,
            timestamp: OffsetDateTime::now_utc(),
            zhl_settings: database.settings.get_zhl_of_user(json.id)?,
            general_settings: database.settings.get_general_of_user(json.id)?,
            segments: bottom_segments,
            deco_gases,
        })?;
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
        end_tissue: res.tissue
    }))
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TissueDiveResult {
    tissue: Tissue,
    total_segments: Vec<(DiveSegment, Gas)>,
    gas_used: HashMap<Gas, usize>,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct SurfaceIntervalInput {
    id: UserID,
    algorithm: Algorithm,
    length: u64,
    start_tissue: Option<Tissue>,
}

#[post("/dive/si")]
pub(crate) async fn surface_interval(
    database: Data<Arc<Database>>,
    json: Json<SurfaceIntervalInput>
) -> actix_web::Result<HttpResponse> {
    let params = DiveRouteParameters {
        segments: vec![
            (
                JSONDiveSegment {
                    segment_type: SegmentType::DiveSegment,
                    start_depth: 0,
                    end_depth: 0,
                    time: json.length as i128,
                    ascent_rate: 1,
                    descent_rate: 1
                },

                JSONGas {
                    o2: 21,
                    he: 0,
                    max_operating_depth: None
                }
                )
        ],
        deco_gases: vec![]
    };

    dive(
        &database,
        &Json(DiveRouteInput {
            id: json.id,
            algorithm: json.algorithm,
            parameters: params,
            start_tissue: json.start_tissue,
            record: false
        })
    ).await
}

async fn get_dive_result<T: DecoAlgorithm>(
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
