use crate::db::schema::{zhl_settings, vpm_settings, gas_plan_settings};
use serde::{Deserialize, Serialize};


#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[table_name = "zhl_settings"]
pub struct ZHLSetting {
    pub id: i32,
    pub gfl: i32,
    pub gfh: i32,
    pub ascent_rate: i32,
    pub descent_rate: i32
}

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[table_name = "vpm_settings"]
pub struct VPMSetting {
    pub id: i32,
}

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[table_name = "gas_plan_settings"]
pub struct GasPlanSetting {
    pub id: i32,
    pub sac_bottom: i32,
    pub sac_deco: i32,
}