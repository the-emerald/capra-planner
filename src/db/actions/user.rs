use diesel::{SqliteConnection, RunQueryDsl};
use crate::db::models;
use crate::db::models::user::{NewUser, User};
use diesel::prelude::*;

pub fn insert_new_user(new_user: &NewUser,
                       conn: &SqliteConnection
) -> Result<models::user::User, diesel::result::Error> {
    use crate::db::schema::users::dsl::*;

    conn.transaction::<User, diesel::result::Error, _>(|| {
        diesel::insert_into(users)
            .values(new_user)
            .execute(conn)?;

        users
            .filter(name.eq(&new_user.name))
            .first::<models::user::User>(conn)
    })
}

pub fn get_user_by_id(id_: i32,
                      conn: &SqliteConnection
) -> Result<Option<models::user::User>, diesel::result::Error> {
    use crate::db::schema::users::dsl::*;

    let user = users
        .filter(id.eq(id_))
        .first::<models::user::User>(conn)
        .optional()?;

    Ok(user)
}

pub fn get_list_all_users(conn: &SqliteConnection) -> Result<Vec<models::user::User>, diesel::result::Error> {
    use crate::db::schema::users::dsl::*;
    users.load::<models::user::User>(conn)
}