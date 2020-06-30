use diesel::SqliteConnection;
use crate::db::models;
use crate::db::models::settings::*;
use diesel::prelude::*;

pub fn get_all_settings_for_user(user: &models::user::User,
                                 conn: &SqliteConnection
) -> Result<(ZHLSettings, VPMSettings, GeneralSettings), diesel::result::Error> {
    let zhl = get_zhl_settings_for_user(user, conn)?;
    let vpm = get_vpm_settings_for_user(user, conn)?;
    let gas_plan = get_general_settings_for_user(user, conn)?;
    Ok((zhl, vpm, gas_plan))
}

pub fn get_zhl_settings_for_user(user: &models::user::User,
                                 conn: &SqliteConnection
) -> Result<ZHLSettings, diesel::result::Error> {
    use crate::db::schema::zhl_settings::dsl::*;
    zhl_settings
        .filter(id.eq(user.current_zhl_settings_id))
        .first::<ZHLSettings>(conn)
}

pub fn update_zhl_settings_for_user(user: &models::user::User,
                                    new: &NewZHLSettings,
                                    conn: &SqliteConnection
) -> Result<(), diesel::result::Error> {
    use crate::db::schema::zhl_settings::dsl::*;

    conn.transaction::<(), diesel::result::Error, _>(|| {
        let candidate_settings = zhl_settings
            .filter(gfl.eq(&new.gfl))
            .filter(gfh.eq(&new.gfh))
            .first::<ZHLSettings>(conn)
            .optional()?;

        let candidate_settings = match candidate_settings {
            Some(t) => { t }, // The system has seen this combination of gfl/gfh before
            None => {
                diesel::insert_into(zhl_settings)
                    .values(new)
                    .execute(conn)?;

                zhl_settings
                    .filter(gfl.eq(&new.gfl))
                    .filter(gfh.eq(&new.gfh))
                    .first::<ZHLSettings>(conn)?
            }
        };
        {
            use crate::db::schema::users::dsl::*;
            diesel::update(
                users.filter(id.eq(&user.id))
            )
                .set(current_zhl_settings_id.eq(candidate_settings.id))
                .execute(conn)?;
        }

        Ok(())
    })
}

pub fn get_vpm_settings_for_user(user: &models::user::User,
                                 conn: &SqliteConnection
) -> Result<VPMSettings, diesel::result::Error> {
    use crate::db::schema::vpm_settings::dsl::*;
    vpm_settings
        .filter(id.eq(user.current_vpm_settings_id))
        .first::<VPMSettings>(conn)
}

pub fn get_general_settings_for_user(user: &models::user::User,
                                     conn: &SqliteConnection
) -> Result<GeneralSettings, diesel::result::Error> {
    use crate::db::schema::general_settings::dsl::*;
    general_settings
        .filter(id.eq(user.current_general_settings_id))
        .first::<GeneralSettings>(conn)
}