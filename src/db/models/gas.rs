use crate::db::schema::{gases};
use crate::db::models::dive::Dive;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Identifiable, Associations)]
#[table_name = "gases"]
#[belongs_to(parent = "Dive", foreign_key = "dive_id")]
pub struct Gas {
    pub id: i32,
    pub dive_id: Option<i32>,
    pub o2: i32,
    pub he: i32,
    pub max_operating_depth: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "gases"]
pub struct NewGas {
    pub dive_id: Option<i32>,
    pub o2: i32,
    pub he: i32,
    pub max_operating_depth: Option<i32>,
}