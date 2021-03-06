use serde::{Deserialize, Serialize};
use capra::common::dive_segment::{SegmentType};
use capra::common::dive_segment;
use std::convert::TryFrom;
use time::Duration;
use crate::result::{ServerDiveSegmentError};

// Represents a DiveSegment sent by JSON.
// Difference(s):
// - `time` is represented in milliseconds instead of Duration
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct DiveSegment {
    pub segment_type: SegmentType,
    pub start_depth: usize,
    pub end_depth: usize,
    pub time: i128,
    // Ascent rate and descent rate are for reaching the current DiveSegment from the previous.
    pub ascent_rate: isize,
    pub descent_rate: isize,
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
