use diesel::SqliteConnection;
use crate::db::models;
use crate::db::models::tissue::{Tissue};
use diesel::prelude::*;
use crate::simplified::SimplifiedTissue;

pub fn get_tissue_of_user(user: &models::user::User,
                          conn: &SqliteConnection
) -> Result<Tissue, diesel::result::Error> {
    use crate::db::schema::tissues::dsl::*;
    tissues
        .filter(id.eq(user.current_tissue_id))
        .first::<Tissue>(conn)
}

pub fn update_tissue_of_user(user: &models::user::User,
                             new_tissue: SimplifiedTissue,
                             conn: &SqliteConnection,
) -> Result<(), diesel::result::Error> {
    use crate::db::schema::tissues::dsl::*;
    
    conn.transaction::<(), diesel::result::Error, _>(|| {
        let candidate_tissue = tissues // TODO: Make this into a macro
            .filter(n2_1.eq(&new_tissue.n2_1))
            .filter(n2_2.eq(&new_tissue.n2_2))
            .filter(n2_3.eq(&new_tissue.n2_3))
            .filter(n2_4.eq(&new_tissue.n2_4))
            .filter(n2_5.eq(&new_tissue.n2_5))
            .filter(n2_6.eq(&new_tissue.n2_6))
            .filter(n2_7.eq(&new_tissue.n2_7))
            .filter(n2_8.eq(&new_tissue.n2_8))
            .filter(n2_9.eq(&new_tissue.n2_9))
            .filter(n2_10.eq(&new_tissue.n2_10))
            .filter(n2_11.eq(&new_tissue.n2_11))
            .filter(n2_12.eq(&new_tissue.n2_12))
            .filter(n2_13.eq(&new_tissue.n2_13))
            .filter(n2_14.eq(&new_tissue.n2_14))
            .filter(n2_15.eq(&new_tissue.n2_15))
            .filter(n2_16.eq(&new_tissue.n2_16))

            .filter(he_1.eq(&new_tissue.he_1))
            .filter(he_2.eq(&new_tissue.he_2))
            .filter(he_3.eq(&new_tissue.he_3))
            .filter(he_4.eq(&new_tissue.he_4))
            .filter(he_5.eq(&new_tissue.he_5))
            .filter(he_6.eq(&new_tissue.he_6))
            .filter(he_7.eq(&new_tissue.he_7))
            .filter(he_8.eq(&new_tissue.he_8))
            .filter(he_9.eq(&new_tissue.he_9))
            .filter(he_10.eq(&new_tissue.he_10))
            .filter(he_11.eq(&new_tissue.he_11))
            .filter(he_12.eq(&new_tissue.he_12))
            .filter(he_13.eq(&new_tissue.he_13))
            .filter(he_14.eq(&new_tissue.he_14))
            .filter(he_15.eq(&new_tissue.he_15))
            .filter(he_16.eq(&new_tissue.he_16))

            .first::<Tissue>(conn)
            .optional()?;

        let candidate_tissue = match candidate_tissue {
            Some(t) => { t },
            None => {
                diesel::insert_into(tissues)
                    .values(new_tissue)
                    .execute(conn)?;

                tissues
                    .filter(n2_1.eq(&new_tissue.n2_1))
                    .filter(n2_2.eq(&new_tissue.n2_2))
                    .filter(n2_3.eq(&new_tissue.n2_3))
                    .filter(n2_4.eq(&new_tissue.n2_4))
                    .filter(n2_5.eq(&new_tissue.n2_5))
                    .filter(n2_6.eq(&new_tissue.n2_6))
                    .filter(n2_7.eq(&new_tissue.n2_7))
                    .filter(n2_8.eq(&new_tissue.n2_8))
                    .filter(n2_9.eq(&new_tissue.n2_9))
                    .filter(n2_10.eq(&new_tissue.n2_10))
                    .filter(n2_11.eq(&new_tissue.n2_11))
                    .filter(n2_12.eq(&new_tissue.n2_12))
                    .filter(n2_13.eq(&new_tissue.n2_13))
                    .filter(n2_14.eq(&new_tissue.n2_14))
                    .filter(n2_15.eq(&new_tissue.n2_15))
                    .filter(n2_16.eq(&new_tissue.n2_16))

                    .filter(he_1.eq(&new_tissue.he_1))
                    .filter(he_2.eq(&new_tissue.he_2))
                    .filter(he_3.eq(&new_tissue.he_3))
                    .filter(he_4.eq(&new_tissue.he_4))
                    .filter(he_5.eq(&new_tissue.he_5))
                    .filter(he_6.eq(&new_tissue.he_6))
                    .filter(he_7.eq(&new_tissue.he_7))
                    .filter(he_8.eq(&new_tissue.he_8))
                    .filter(he_9.eq(&new_tissue.he_9))
                    .filter(he_10.eq(&new_tissue.he_10))
                    .filter(he_11.eq(&new_tissue.he_11))
                    .filter(he_12.eq(&new_tissue.he_12))
                    .filter(he_13.eq(&new_tissue.he_13))
                    .filter(he_14.eq(&new_tissue.he_14))
                    .filter(he_15.eq(&new_tissue.he_15))
                    .filter(he_16.eq(&new_tissue.he_16))

                    .first::<Tissue>(conn)?
            }
        };

        // Update the user
        {
            use crate::db::schema::users::dsl::*;
            diesel::update(
                users.filter(id.eq(&user.id))
            )
                .set(current_tissue_id.eq(candidate_tissue.id))
                .execute(conn)?;
        }

        Ok(())
    })

        
}