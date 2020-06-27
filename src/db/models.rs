use crate::db::schema::{users, zhl_settings};

#[derive(Queryable, Identifiable, Associations)]
#[table_name = "users"]
#[belongs_to(parent = "ZHLSettings", foreign_key = "current_zhl_setting_id")]
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
pub struct ZHLSettings {
    pub id: i32,
    pub gfl: i32,
    pub gfh: i32,
    pub ascent_rate: i32,
    pub descent_rate: i32
}