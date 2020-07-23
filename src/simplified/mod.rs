use crate::db::models;
use crate::db::models::user::User;
use serde::{Serialize, Deserialize};
use crate::db::schema::{tissues};
use crate::db::models::tissue::Tissue;
use crate::db::models::settings::{ZHLSettings, VPMSettings, GeneralSettings};
use crate::json_repr;
use std::convert::{TryFrom, TryInto};
use crate::result::ZHLSettingError;

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
            name: value.name
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Insertable)]
#[table_name = "tissues"] // Double duty!
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

impl From<capra::deco::tissue::Tissue> for SimplifiedTissue {
    fn from(value: capra::deco::tissue::Tissue) -> Self {
        Self {
            n2_1: value.p_n2()[0],
            n2_2: value.p_n2()[1],
            n2_3: value.p_n2()[2],
            n2_4: value.p_n2()[3],
            n2_5: value.p_n2()[4],
            n2_6: value.p_n2()[5],
            n2_7: value.p_n2()[6],
            n2_8: value.p_n2()[7],
            n2_9: value.p_n2()[8],
            n2_10: value.p_n2()[9],
            n2_11: value.p_n2()[10],
            n2_12: value.p_n2()[11],
            n2_13: value.p_n2()[12],
            n2_14: value.p_n2()[13],
            n2_15: value.p_n2()[14],
            n2_16: value.p_n2()[15],
            he_1: value.p_he()[0],
            he_2: value.p_he()[1],
            he_3: value.p_he()[2],
            he_4: value.p_he()[3],
            he_5: value.p_he()[4],
            he_6: value.p_he()[5],
            he_7: value.p_he()[6],
            he_8: value.p_he()[7],
            he_9: value.p_he()[8],
            he_10: value.p_he()[9],
            he_11: value.p_he()[10],
            he_12: value.p_he()[11],
            he_13: value.p_he()[12],
            he_14: value.p_he()[13],
            he_15: value.p_he()[14],
            he_16: value.p_he()[15],
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum ZHLVariant {
    B,
    C
}

impl From<ZHLVariant> for String {
    fn from(value: ZHLVariant) -> Self {
        match value {
            ZHLVariant::B => { String::from("B") },
            ZHLVariant::C => { String::from("C") },
        }
    }
}

impl TryFrom<String> for ZHLVariant {
    type Error = ZHLSettingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "B" => { Ok(ZHLVariant::B) }
            "C" => { Ok(ZHLVariant::C) }
            _ => { Err(ZHLSettingError::ConversionError) }
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct SimplifiedZHLSettings {
    pub variant: ZHLVariant,
    pub gfl: i32,
    pub gfh: i32,
}

impl TryFrom<models::settings::ZHLSettings> for SimplifiedZHLSettings {
    type Error = ZHLSettingError;

    fn try_from(value: ZHLSettings) -> Result<Self, Self::Error> {
        Ok(
            Self {
                variant: value.variant.try_into()?,
                gfl: value.gfl,
                gfh: value.gfh
            }
        )
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct SimplifiedVPMSettings {

}

impl From<models::settings::VPMSettings> for SimplifiedVPMSettings {
    fn from(_value: VPMSettings) -> Self {
        Self {

        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SimplifiedDive {
    pub segments: Vec<(json_repr::dive_segment::DiveSegment, json_repr::gas::Gas)>,
    pub deco_gases: Vec<json_repr::gas::Gas>,
}