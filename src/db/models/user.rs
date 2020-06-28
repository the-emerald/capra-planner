use crate::db::schema::{users};
use crate::db::models::settings::*;
use crate::db::models::tissue::Tissue;

#[derive(Queryable, Identifiable, Associations)]
#[table_name = "users"]
#[belongs_to(parent = "Tissue", foreign_key = "current_tissue_id")]
#[belongs_to(parent = "ZHLSetting", foreign_key = "current_zhl_settings_id")]
#[belongs_to(parent = "VPMSetting", foreign_key = "current_vpm_settings_id")]
#[belongs_to(parent = "GasPlanSetting", foreign_key = "current_gas_plan_settings_id")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub current_tissue_id: i32,
    pub current_zhl_settings_id: i32,
    pub current_vpm_settings_id: i32,
    pub current_gas_plan_settings_id: i32
}