use crate::db::schema::{zhl_settings, vpm_settings, general_settings};
use serde::{Deserialize, Serialize};
use crate::simplified::{SimplifiedZHLSettings, SimplifiedGeneralSettings};


#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[table_name = "zhl_settings"]
pub struct ZHLSettings {
    pub id: i32,
    pub subtype: String,
    pub gfl: i32,
    pub gfh: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "zhl_settings"]
pub struct NewZHLSettings {
    pub subtype: String,
    pub gfl: i32,
    pub gfh: i32,
}

impl From<SimplifiedZHLSettings> for NewZHLSettings {
    fn from(value: SimplifiedZHLSettings) -> Self {
        Self {
            subtype: String::from(value.subtype),
            gfl: value.gfl,
            gfh: value.gfh
        }
    }
}

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[table_name = "vpm_settings"]
pub struct VPMSettings {
    pub id: i32,
}

// #[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
// #[table_name = "vpm_settings"]
// pub struct NewVPMSettings {
//
// }
//
// impl From<SimplifiedVPMSettings> for NewVPMSettings {
//     fn from(value: SimplifiedVPMSettings) -> Self {
//         Self {
//         }
//     }
// }

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[table_name = "general_settings"]
pub struct GeneralSettings {
    pub id: i32,
    pub sac_bottom: i32,
    pub sac_deco: i32,
    pub ascent_rate: i32,
    pub descent_rate: i32,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "general_settings"]
pub struct NewGeneralSettings {
    pub sac_bottom: i32,
    pub sac_deco: i32,
    pub ascent_rate: i32,
    pub descent_rate: i32,
}

impl From<SimplifiedGeneralSettings> for NewGeneralSettings {
    fn from(value: SimplifiedGeneralSettings) -> Self {
        Self {
            sac_bottom: value.sac_bottom,
            sac_deco: value.sac_deco,
            ascent_rate: value.ascent_rate,
            descent_rate: value.descent_rate
        }
    }
}