use crate::db::models;
use crate::db::models::user::User;
use serde::{Serialize, Deserialize};
use crate::db::models::tissue::Tissue;
use crate::db::models::settings::{ZHLSettings, VPMSettings, GeneralSettings};

// A simplified user that only contains the id and name field.
#[derive(Serialize, Deserialize)]
pub(crate) struct SimplifiedUser {
    id: i32,
    name: String,
}

impl From<models::user::User> for SimplifiedUser {
    fn from(value: User) -> Self {
        SimplifiedUser {
            id: value.id,
            name: value.name.clone()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SimplifiedTissue {
    // Nitrogen loadings
    pub n2_1: f64,
    pub n2_2: f64,
    pub n2_3: f64,
    pub n2_4: f64,
    pub n2_5: f64,
    pub n2_6: f64,
    pub n2_7: f64,
    pub n2_8: f64,
    pub n2_9: f64,
    pub n2_10: f64,
    pub n2_11: f64,
    pub n2_12: f64,
    pub n2_13: f64,
    pub n2_14: f64,
    pub n2_15: f64,
    pub n2_16: f64,

    // Helium loadings
    pub he_1: f64,
    pub he_2: f64,
    pub he_3: f64,
    pub he_4: f64,
    pub he_5: f64,
    pub he_6: f64,
    pub he_7: f64,
    pub he_8: f64,
    pub he_9: f64,
    pub he_10: f64,
    pub he_11: f64,
    pub he_12: f64,
    pub he_13: f64,
    pub he_14: f64,
    pub he_15: f64,
    pub he_16: f64,
}

impl From<models::tissue::Tissue> for SimplifiedTissue {
    fn from(value: Tissue) -> Self {
        Self {
            n2_1: value.n2_1,
            n2_2: value.n2_2,
            n2_3: value.n2_3,
            n2_4: value.n2_4,
            n2_5: value.n2_5,
            n2_6: value.n2_6,
            n2_7: value.n2_7,
            n2_8: value.n2_8,
            n2_9: value.n2_9,
            n2_10: value.n2_10,
            n2_11: value.n2_11,
            n2_12: value.n2_12,
            n2_13: value.n2_13,
            n2_14: value.n2_14,
            n2_15: value.n2_15,
            n2_16: value.n2_16,
            he_1: value.he_1,
            he_2: value.he_2,
            he_3: value.he_3,
            he_4: value.he_4,
            he_5: value.he_5,
            he_6: value.he_6,
            he_7: value.he_7,
            he_8: value.he_8,
            he_9: value.he_9,
            he_10: value.he_10,
            he_11: value.he_11,
            he_12: value.he_12,
            he_13: value.he_13,
            he_14: value.he_14,
            he_15: value.he_15,
            he_16: value.he_16
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SimplifiedZHLSettings {
    pub gfl: i32,
    pub gfh: i32,
}

impl From<models::settings::ZHLSettings> for SimplifiedZHLSettings {
    fn from(value: ZHLSettings) -> Self {
        Self {
            gfl: value.gfl,
            gfh: value.gfh,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SimplifiedVPMSettings {

}

impl From<models::settings::VPMSettings> for SimplifiedVPMSettings {
    fn from(_value: VPMSettings) -> Self {
        Self {

        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SimplifiedGeneralSettings {
    pub sac_bottom: i32,
    pub sac_deco: i32,
    pub ascent_rate: i32,
    pub descent_rate: i32,
}

impl From<models::settings::GeneralSettings> for SimplifiedGeneralSettings {
    fn from(value: GeneralSettings) -> Self {
        Self {
            sac_bottom: value.sac_bottom,
            sac_deco: value.sac_deco,
            ascent_rate: value.ascent_rate,
            descent_rate: value.descent_rate
        }
    }
}