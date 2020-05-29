use serde::{Deserialize, Serialize};
use capra::common::dive_segment::{SegmentType, DiveSegmentError};
use capra::common::dive_segment;
use std::convert::TryFrom;
use time::Duration;
use actix_web::{ResponseError, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::dev::HttpResponseBuilder;

// Represents a DiveSegment sent by JSON.
// Difference(s):
// - `time` is represented in milliseconds instead of Duration
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub(crate) struct DiveSegment {
    segment_type: SegmentType,
    start_depth: usize,
    end_depth: usize,
    time: i128,
    // Ascent rate and descent rate are for reaching the current DiveSegment from the previous.
    ascent_rate: isize,
    descent_rate: isize,
}

impl From<dive_segment::DiveSegment> for DiveSegment {
    fn from(value: dive_segment::DiveSegment) -> Self {
        Self {
            segment_type: value.segment_type(),
            start_depth: value.start_depth(),
            end_depth: value.end_depth(),
            time: value.time().whole_milliseconds(),
            ascent_rate: value.ascent_rate(),
            descent_rate: value.descent_rate()
        }
    }
}

impl TryFrom<DiveSegment> for dive_segment::DiveSegment {
    type Error = ServerDiveSegmentError;

    fn try_from(value: DiveSegment) -> Result<Self, Self::Error> {
        Ok(
            Self::new(
            value.segment_type,
            value.start_depth,
            value.end_depth,
            Duration::milliseconds(value.time as i64),
            value.ascent_rate,
            value.descent_rate
            )?
        )
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ServerDiveSegmentError {
    #[error(transparent)]
    IncorrectSegmentTypeError(#[from] DiveSegmentError)
}

impl ResponseError for ServerDiveSegmentError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .body(self.to_string())
    }
}