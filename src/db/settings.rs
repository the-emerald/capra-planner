use crate::db::users::UserID;
use crate::db::DatabaseError;
use capra_core::deco::zhl16;
use capra_core::deco::zhl16::Variant;
use serde::{Deserialize, Serialize};
use sled::{Db, Tree};

#[derive(Clone, Debug)]
pub struct ZHLSettingsTree(Tree, Db);

impl ZHLSettingsTree {
    pub fn open(database: &Db) -> sled::Result<Self> {
        Ok(Self(database.open_tree("zhl_settings")?, database.clone()))
    }
}

// ZHL Settings (value)
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ZHLSettings {
    pub variant: zhl16::Variant,
    pub gfl: u8,
    pub gfh: u8,
}

impl Default for ZHLSettings {
    fn default() -> Self {
        Self {
            variant: Variant::B,
            gfl: 50,
            gfh: 70,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GeneralSettingsTree(Tree, Db);

impl GeneralSettingsTree {
    pub fn open(database: &Db) -> sled::Result<Self> {
        Ok(Self(
            database.open_tree("general_settings")?,
            database.clone(),
        ))
    }
}

// TODO: Check whether these integer sizes are okay
// General settings (value)
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct GeneralSettings {
    pub sac_bottom: u16,
    pub sac_deco: u16,
    pub ascent_rate: i16,
    pub descent_rate: i16,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            sac_bottom: 20,
            sac_deco: 15,
            ascent_rate: -10,
            descent_rate: 20,
        }
    }
}

// Key: username, value: relevant settings
#[derive(Clone, Debug)]
pub struct SettingsTree {
    zhl: ZHLSettingsTree,
    // vpm tree: coming soon(tm)
    general: GeneralSettingsTree,
}

impl SettingsTree {
    pub fn open(database: &Db) -> sled::Result<Self> {
        Ok(Self {
            zhl: ZHLSettingsTree::open(&database)?,
            general: GeneralSettingsTree::open(&database)?,
        })
    }

    pub fn initialise_user(&self, id: UserID) -> Result<bool, DatabaseError> {
        let old_general = self.general.0.insert(
            serde_json::to_string(&id)?.as_bytes(),
            serde_json::to_string(&GeneralSettings::default())?.as_bytes(),
        )?;

        let old_zhl = self.zhl.0.insert(
            serde_json::to_string(&id)?.as_bytes(),
            serde_json::to_string(&ZHLSettings::default())?.as_bytes(),
        )?;

        Ok(old_general.is_some() || old_zhl.is_some())
    }
}