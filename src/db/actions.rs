use diesel::{SqliteConnection, RunQueryDsl};
use crate::db::models;
use crate::db::models::user::NewUser;
use diesel::prelude::*;

pub fn insert_new_user(new_user: &NewUser,
                       conn: &SqliteConnection
) -> Result<models::user::User, diesel::result::Error> {
    // Unfortunately SQLite does not allow for get_result(conn), so we are forced to make a query
    // to insert the new user and then fetch the row we just created.

    use crate::db::schema::users::dsl::*;

    // Insert
    diesel::insert_into(users)
        .values(new_user)
        .execute(conn)?;

    // Fetch the new user
    let user = users
        .filter(name.eq(&new_user.name))
        .first::<models::user::User>(conn)?;

    Ok(user)
}