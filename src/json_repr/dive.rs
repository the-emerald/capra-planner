use crate::db::dives::{Dive, PlanType};
use crate::db::settings::{GeneralSettings, ZHLSettings};
use crate::db::users::UserID;
use crate::json_repr::dive_segment::JSONDiveSegment;
use crate::json_repr::gas::JSONGas;
use capra_core::deco::Tissue;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

// Value component of the dives tree
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JSONDive {
    pub user: UserID,
    pub plan_type: PlanType,
    pub tissue_before: Tissue, // Before surface interval
    pub surface_interval: u64,
    pub timestamp: OffsetDateTime,

    // Snapshot of ZHL and General settings used at that time.
    pub zhl_settings: ZHLSettings,
    pub general_settings: GeneralSettings,

    // Actual information about the dive.
    pub segments: Vec<(JSONDiveSegment, JSONGas)>,
    pub deco_gases: Vec<JSONGas>,
}

impl From<Dive> for JSONDive {
    fn from(val: Dive) -> Self {
        Self {
            user: val.user,
            plan_type: val.plan_type,
            tissue_before: val.tissue_before,
            surface_interval: val.surface_interval,
            timestamp: val.timestamp,
            zhl_settings: val.zhl_settings,
            general_settings: val.general_settings,
            segments: val
                .segments
                .into_iter()
                .map(|(d, g)| (d.into(), g.into()))
                .collect(),
            deco_gases: val
                .deco_gases
                .into_iter()
                .map(|t| t.into())
                .collect(),
        }
    }
}
