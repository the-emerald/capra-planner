use crate::db::schema::{users, zhl_settings, vpm_settings, gas_plan_settings};

#[derive(Queryable, Identifiable, Associations)]
#[table_name = "users"]
#[belongs_to(parent = "ZHLSetting", foreign_key = "current_zhl_setting_id")]
#[belongs_to(parent = "VPMSetting", foreign_key = "current_vpm_setting_id")]
#[belongs_to(parent = "GasPlanSetting", foreign_key = "current_gas_plan_setting_id")]

pub struct User {
    pub id: i32,
    pub name: String,
    pub current_tissue_id: i32,
    pub current_zhl_setting_id: i32,
    pub current_vpm_setting_id: i32,
    pub current_gas_plan_setting_id: i32
}

#[derive(Queryable, Identifiable)]
#[table_name = "zhl_settings"]
pub struct ZHLSetting {
    pub id: i32,
    pub gfl: i32,
    pub gfh: i32,
    pub ascent_rate: i32,
    pub descent_rate: i32
}

#[derive(Queryable, Identifiable)]
#[table_name = "vpm_settings"]
pub struct VPMSetting {
    pub id: i32,
}

#[derive(Queryable, Identifiable)]
#[table_name = "gas_plan_settings"]
pub struct GasPlanSetting {
    pub id: i32,
    pub sac_bottom: i32,
    pub sac_deco: i32,
}