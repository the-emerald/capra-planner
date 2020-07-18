use crate::db::schema::{dives};
use crate::db::models::settings::*;
use crate::db::models::tissue::Tissue;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable, Associations)]
#[table_name = "dives"]
#[belongs_to(parent = "Tissue", foreign_key = "tissue_before_id")]
#[belongs_to(parent = "ZHLSettings", foreign_key = "zhl_settings_id")]
#[belongs_to(parent = "VPMSettings", foreign_key = "vpm_settings_id")]
#[belongs_to(parent = "GeneralSettings", foreign_key = "general_settings_id")]
pub struct Dive {
    pub id: i32,
    pub user_id: i32,
    pub tissue_before_id: i32,
    pub timestamp: NaiveDateTime,
    pub executed: i32,
    pub zhl_settings_id: i32,
    pub vpm_settings_id: i32,
    pub general_settings_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "dives"]
pub struct NewDive {
    pub user_id: i32,
    pub tissue_before_id: i32,
    pub timestamp: NaiveDateTime,
    pub executed: i32,
    pub zhl_settings_id: i32,
    pub vpm_settings_id: i32,
    pub general_settings_id: i32,
}