use crate::db::settings::{GeneralSettings, ZHLSettings};
use crate::db::users::UserID;
use capra_core::deco::Tissue;
use serde::{Deserialize, Serialize};
use sled::{Db, Tree};
use time::PrimitiveDateTime;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum DiveType {
    Plan,
    Execution,
}

// Value component of the dives tree
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Dive {
    pub user: UserID,
    pub variant: DiveType,
    pub tissue_before: Tissue,
    pub timestamp: PrimitiveDateTime, // todo: take a look at this?
    pub zhl_settings: ZHLSettings,
    pub general_settings: GeneralSettings,
}

// Key: ID, value: Dive (parameters)
#[derive(Clone, Debug)]
pub struct DivesTree(Tree, Db);

impl DivesTree {
    pub fn open(database: &Db) -> sled::Result<Self> {
        Ok(Self(database.open_tree("dives")?, database.clone()))
    }
}
