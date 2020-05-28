use serde::{Deserialize, Serialize};
use capra::common::dive_segment::{SegmentType, DiveSegmentError};
use capra::common::dive_segment;
use std::convert::TryFrom;
use time::Duration;

#[derive(Serialize, Deserialize)]
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
    type Error = DiveSegmentError;

    fn try_from(value: DiveSegment) -> Result<Self, Self::Error> {
        Self::new(
            value.segment_type,
            value.start_depth,
            value.end_depth,
            Duration::milliseconds(value.time as i64),
            value.ascent_rate,
            value.descent_rate
        )
    }
}