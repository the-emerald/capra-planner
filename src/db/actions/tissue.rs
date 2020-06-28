use diesel::SqliteConnection;
use crate::db::models;
use crate::db::models::tissue::Tissue;
use diesel::prelude::*;

pub fn get_tissue_of_user(user: &models::user::User,
                          conn: &SqliteConnection
) -> Result<Tissue, diesel::result::Error> {
    use crate::db::schema::tissues::dsl::*;
    tissues
        .filter(id.eq(user.current_tissue_id))
        .first::<Tissue>(conn)
}