use crate::db::schema::{dives};
use crate::db::models::settings::*;
use crate::db::models::tissue::Tissue;
use diesel::sql_types::Timestamp;

#[derive(Queryable, Identifiable, Associations)]
#[table_name = "dives"]
#[belongs_to(parent = "Tissue", foreign_key = "tissue_before_id")]
#[belongs_to(parent = "ZHLSetting", foreign_key = "zhl_settings_id")]
#[belongs_to(parent = "VPMSetting", foreign_key = "vpm_settings_id")]
#[belongs_to(parent = "GasPlanSetting", foreign_key = "gas_plan_settings_id")]
pub struct Dive {
    pub id: i32,
    pub user_id: i32,
    pub tissue_before_id: i32,
    pub timestamp: Timestamp,
    pub zhl_settings_id: i32,
    pub vpm_settings_id: i32,
    pub gas_plan_settings_id: i32,
}
