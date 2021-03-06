use crate::db::models::dive::Dive;
use crate::db::models::gas::Gas;
use crate::db::schema::{segments};
use serde::{Serialize, Deserialize};

#[derive(Queryable, Identifiable, Associations)]
#[table_name = "segments"]
#[belongs_to(parent = "Dive", foreign_key = "dive_id")]
#[belongs_to(parent = "Gas", foreign_key = "gas_id")]
pub struct Segment {
    pub id: i32,
    pub dive_id: i32,
    pub start_depth: i32,
    pub end_depth: i32,
    pub time: i32,
    pub ascent_rate: i32,
    pub descent_rate: i32,
    pub gas_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "segments"]
pub struct NewSegment {
    pub dive_id: i32,
    pub start_depth: i32,
    pub end_depth: i32,
    pub time: i32,
    pub ascent_rate: i32,
    pub descent_rate: i32,
    pub gas_id: i32,
}