use sled::{Tree, Db};
use crate::db::users::UserID;
use capra_core::deco::Tissue;
use time::PrimitiveDateTime;
use crate::db::settings::{ZHLSettings, GeneralSettings};

#[derive(Copy, Clone, Debug)]
pub enum DiveType {
    Plan,
    Execution
}

// Value component of the dives tree
#[derive(Copy, Clone, Debug)]
pub struct Dive {
    user: UserID,
    variant: DiveType,
    tissue_before: Tissue,
    timestamp: PrimitiveDateTime, // todo: take a look at this?
    zhl_settings: ZHLSettings,
    general_settings: GeneralSettings,
}

// Key: ID, value: Dive (parameters)
#[derive(Clone, Debug)]
pub struct DivesTree(Tree);

impl DivesTree {
    pub fn open(database: &Db) -> sled::Result<Self> {
        Ok(Self(database.open_tree("dives")?))
    }
}
