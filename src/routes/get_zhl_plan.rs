use actix_web::{post, web, HttpResponse};
use serde::{Serialize, Deserialize};
use crate::json_repr;
use std::convert::TryInto;
use crate::json_repr::gas::ServerGasError;
use capra::deco::zhl16::ZHL16;
use capra::common::gas::Gas;
use capra::deco::zhl16::util::{ZHL16B_N2_A, ZHL16B_N2_B, ZHL16B_N2_HALFLIFE, ZHL16B_HE_A, ZHL16B_HE_B, ZHL16B_HE_HALFLIFE};
use capra::planning::modes::open_circuit::OpenCircuit;
use capra::planning::DivePlan;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct GetZHLPlanInput {
    gfl: usize,
    gfh: usize,

    ascent_rate: isize,
    descent_rate: isize,

    sac_bottom: usize,
    sac_deco: usize,

    segments: Vec<(json_repr::dive_segment::DiveSegment, json_repr::gas::Gas)>,
    deco_gases: Vec<json_repr::gas::Gas>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct GetZHLPlanOutput {
    segments: Vec<(json_repr::dive_segment::DiveSegment, json_repr::gas::Gas)>,
    gas_used: HashMap<json_repr::gas::Gas, usize>,
}

#[post("zhl/get_plan")]
pub(crate) async fn route(input: web::Json<GetZHLPlanInput>) -> actix_web::Result<HttpResponse> {
    // Convert to capra dive segments
    let mut c_segments: Vec<(capra::common::dive_segment::DiveSegment, capra::common::gas::Gas)> = Vec::new();
    for s in &input.segments {
        let ds: capra::common::dive_segment::DiveSegment = s.0.try_into()?;
        let g: capra::common::gas::Gas = s.1.try_into()?;
        c_segments.push((ds, g));
    }

    // Convert to capra gases
    let c_deco_gases = input.deco_gases
        .iter()
        .cloned()
        .map(|x| Ok((x.try_into()?, x.max_op_depth())))
        .collect::<Result<Vec<(capra::common::gas::Gas, Option<usize>)>, ServerGasError>>()?;

    // Construct dive plan
    let zhl16 = ZHL16::new(
        &Gas::new(21, 0, 79).map_err(|x| ServerGasError::FractionError(x))?,
        ZHL16B_N2_A, ZHL16B_N2_B, ZHL16B_N2_HALFLIFE, ZHL16B_HE_A, ZHL16B_HE_B, ZHL16B_HE_HALFLIFE,
        input.gfl, input.gfh
    );

    let open_circuit = OpenCircuit::new(
        zhl16,
        &c_deco_gases,
        &c_segments,
        input.ascent_rate,
        input.descent_rate,
        1000.0,
        input.sac_bottom,
        input.sac_deco
    );

    // Execute plan
    let plan = open_circuit.plan();

    let segments: Vec<(json_repr::dive_segment::DiveSegment, json_repr::gas::Gas)> = plan.total_segments()
        .iter()
        .cloned()
        .map(|x| (x.0.into(), x.1.into()))
        .collect();

    let mut gas_used: HashMap<json_repr::gas::Gas, usize> = HashMap::new();
    for (key, value) in plan.gas_used().iter() {
        gas_used.insert((*key).into(), *value);
    }

    Ok(
        HttpResponse::Ok()
            .json(GetZHLPlanOutput {
                segments,
                gas_used
            })
    )
}