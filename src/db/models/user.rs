use crate::db::schema::{users};
use crate::db::models::settings::*;
use crate::db::models::tissue::Tissue;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[table_name = "users"]
#[belongs_to(parent = "Tissue", foreign_key = "current_tissue_id")]
#[belongs_to(parent = "ZHLSettings", foreign_key = "current_zhl_settings_id")]
#[belongs_to(parent = "VPMSettings", foreign_key = "current_vpm_settings_id")]
#[belongs_to(parent = "GeneralSettings", foreign_key = "current_general_settings_id")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub current_tissue_id: i32,
    pub current_zhl_settings_id: i32,
    pub current_vpm_settings_id: i32,
    pub current_general_settings_id: i32
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
}