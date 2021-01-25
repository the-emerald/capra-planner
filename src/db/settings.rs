use sled::{Tree, Db};
use capra_core::deco::zhl16;

#[derive(Clone, Debug)]
pub struct ZHLSettingsTree(Tree);

impl ZHLSettingsTree {
    pub fn open(database: &Db) -> sled::Result<Self> {
        Ok(Self(database.open_tree("zhl_settings")?))
    }
}

// ZHL Settings (value)
#[derive(Copy, Clone, Debug)]
pub struct ZHLSettings {
    variant: zhl16::Variant,
    gfl: u8,
    gfh: u8,
}

#[derive(Clone, Debug)]
pub struct GeneralSettingsTree(Tree);

impl GeneralSettingsTree {
    pub fn open(database: &Db) -> sled::Result<Self> {
        Ok(Self(database.open_tree("general_settings")?))
    }
}

// TODO: Check whether these integer sizes are okay
// General settings (value)
#[derive(Copy, Clone, Debug)]
pub struct GeneralSettings {
    sac_bottom: u16,
    sac_deco: u16,
    ascent_rate: i16,
    descent_rate: i16,
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
        Ok(
            Self {
                zhl: ZHLSettingsTree::open(&database)?,
                general: GeneralSettingsTree::open(&database)?
            }
        )
    }
}
