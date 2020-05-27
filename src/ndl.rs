use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use capra::common::gas::Gas;
use capra::deco::zhl16::util::{ZHL16B_N2_A, ZHL16B_N2_B, ZHL16B_N2_HALFLIFE, ZHL16B_HE_A, ZHL16B_HE_B, ZHL16B_HE_HALFLIFE};
use capra::deco::zhl16::ZHL16;
use capra::common::dive_segment::{DiveSegment, SegmentType};
use capra::common::time_taken;
use capra::deco::deco_algorithm::DecoAlgorithm;
use time::Duration;

#[derive(Deserialize)]
pub(crate) struct DiveParameters {
    depth: usize,
    time: usize,
}

#[post("ndl")]
pub(crate) async fn ndl(info: web::Json<DiveParameters>) -> HttpResponse {
    let air = Gas::new(21, 0, 79).unwrap();

    let mut zhl16 = ZHL16::new(
        &Gas::new(21, 0, 79).unwrap(), // This shouldn't error
        ZHL16B_N2_A, ZHL16B_N2_B, ZHL16B_N2_HALFLIFE, ZHL16B_HE_A, ZHL16B_HE_B, ZHL16B_HE_HALFLIFE, 100, 100);

    let one = DiveSegment::new(
        SegmentType::AscDesc,
        0,
        info.depth,
        time_taken(-10, 0, info.depth),
        -10,
        20
    ).unwrap();

    zhl16.add_dive_segment(&one, &air, 10.0);

    let bottom = DiveSegment::new(
        SegmentType::DiveSegment,
        info.depth,
        info.depth,
        Duration::minutes(info.time as i64),
        -10,
        20
    ).unwrap();

    zhl16.add_dive_segment(&bottom, &air, 10.0);

    let no_deco_time = zhl16.get_stops(-10, 20, &air, 10.0);
    match no_deco_time[0].get_segment_type() {
        SegmentType::NoDeco => {
            HttpResponse::Ok()
                .content_type("plain/text")
                .header("X-Hdr", "ndl")
                .body(format!("{}", no_deco_time[0].get_time().whole_minutes()))
        }
        _ => {
            HttpResponse::Ok()
                .content_type("plain/text")
                .header("X-Hdr", "ndl")
                .body("deco")
        }
    }
}