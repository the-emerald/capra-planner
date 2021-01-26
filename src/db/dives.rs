use crate::db::settings::{GeneralSettings, ZHLSettings};
use crate::db::users::UserID;
use crate::db::DatabaseError;
use capra_core::common::{DiveSegment, Gas};
use capra_core::deco::Tissue;
use serde::{Deserialize, Serialize};
use sled::{Db, Tree};
use time::PrimitiveDateTime;

#[derive(Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct DiveID(pub u64);

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum PlanType {
    #[serde(alias = "PLAN")]
    Plan,
    #[serde(alias = "EXECUTE")]
    Execution,
}

// Value component of the dives tree
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dive {
    pub user: UserID,
    pub plan_type: PlanType,
    pub tissue_before: Tissue,
    pub timestamp: PrimitiveDateTime, // TODO: Make sure this type is appropriate

    // Snapshot of ZHL and General settings used at that time.
    pub zhl_settings: ZHLSettings,
    pub general_settings: GeneralSettings,

    // Actual information about the dive.
    pub segments: Vec<(DiveSegment, Gas)>,
    pub deco_gases: Vec<(Gas, Option<usize>)>,
}

// Key: ID, value: Dive (parameters)
#[derive(Clone, Debug)]
pub struct DivesTree(Tree, Db);

impl DivesTree {
    pub fn open(database: &Db) -> sled::Result<Self> {
        Ok(Self(database.open_tree("dives")?, database.clone()))
    }

    pub fn add_dive(&self, dive: &Dive) -> Result<DiveID, DatabaseError> {
        let id = DiveID(self.1.generate_id()?);
        self.0
            .insert(serde_json::to_vec(&id)?, serde_json::to_vec(dive)?)?;

        Ok(id)
    }

    pub fn dives_iter(&self) -> impl Iterator<Item = Result<(DiveID, Dive), DatabaseError>> {
        self.0.iter().map(|x| {
            let id: DiveID = serde_json::from_slice(&*x.clone()?.0)?;
            let d: Dive = serde_json::from_slice(&*x?.1)?;
            Ok((id, d))
        })
    }
}
