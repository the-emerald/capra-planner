use actix_web::{post, web, HttpResponse};
use serde::{Serialize, Deserialize};
use capra::gas;
use capra::common::gas::Gas;
use capra::deco::zhl16::util::{ZHL16B_N2_A, ZHL16B_N2_B, ZHL16B_N2_HALFLIFE, ZHL16B_HE_A, ZHL16B_HE_B, ZHL16B_HE_HALFLIFE};
use capra::deco::zhl16::ZHL16;
use capra::common::dive_segment::{DiveSegment, SegmentType};
use capra::common::time_taken;
use capra::deco::deco_algorithm::DecoAlgorithm;
use time::Duration;

#[derive(Serialize, Deserialize)]
pub(crate) struct NDLInput {
    depth: usize,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NDLOutput {
    deco_reached: bool,
    infinite: bool,
    time_remaining: usize
}

#[post("ndl")]
pub(crate) async fn route(input: web::Json<NDLInput>) -> HttpResponse {
    let air = gas!(21, 0);

    let mut zhl16 = ZHL16::new(
        &air, // This shouldn't error
        ZHL16B_N2_A, ZHL16B_N2_B, ZHL16B_N2_HALFLIFE, ZHL16B_HE_A, ZHL16B_HE_B, ZHL16B_HE_HALFLIFE, 100, 100);

    let one = DiveSegment::new(
        SegmentType::AscDesc,
        0,
        input.depth,
        time_taken(-10, 0, input.depth),
        -10,
        20
    ).unwrap();

    zhl16.add_dive_segment(&one, &air, 10.0);

    let no_deco_time = zhl16.get_stops(-10, 20, &air, 10.0);
    match no_deco_time[0].segment_type() {
        SegmentType::NoDeco => {
            if no_deco_time[0].time().whole_minutes() as usize == usize::MAX {
                HttpResponse::Ok()
                    .json(NDLOutput {
                        deco_reached: false,
                        infinite: true,
                        time_remaining: 0
                    })
            }
            else {
                HttpResponse::Ok()
                    .json(NDLOutput {
                        deco_reached: false,
                        infinite: false,
                        time_remaining: no_deco_time[0].time().whole_minutes() as usize
                    })
            }

        }
        _ => {
            HttpResponse::Ok()
                .json(NDLOutput {
                    deco_reached: true,
                    infinite: false,
                    time_remaining: 0
                })
        }
    }
}