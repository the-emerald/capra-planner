use crate::db::schema::{gases};
use crate::db::models::dive::Dive;

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