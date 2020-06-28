use diesel::SqliteConnection;
use crate::db::models;
use crate::db::models::settings::*;
use diesel::prelude::*;

pub fn get_all_settings_for_user(user: &models::user::User,
                                 conn: &SqliteConnection
) -> Result<(ZHLSetting, VPMSetting, GasPlanSetting), diesel::result::Error> {
    let zhl = get_zhl_settings_for_user(user, conn)?;
    let vpm = get_vpm_settings_for_user(user, conn)?;
    let gas_plan = get_gas_plan_settings_for_user(user, conn)?;
    Ok((zhl, vpm, gas_plan))
}

pub fn get_zhl_settings_for_user(user: &models::user::User,
                                 conn: &SqliteConnection
) -> Result<ZHLSetting, diesel::result::Error> {
    use crate::db::schema::zhl_settings::dsl::*;
    zhl_settings
        .filter(id.eq(user.current_zhl_settings_id))
        .first::<ZHLSetting>(conn)
}

pub fn get_vpm_settings_for_user(user: &models::user::User,
                                 conn: &SqliteConnection
) -> Result<VPMSetting, diesel::result::Error> {
    use crate::db::schema::vpm_settings::dsl::*;
    vpm_settings
        .filter(id.eq(user.current_vpm_settings_id))
        .first::<VPMSetting>(conn)
}

pub fn get_gas_plan_settings_for_user(user: &models::user::User,
                                 conn: &SqliteConnection
) -> Result<GasPlanSetting, diesel::result::Error> {
    use crate::db::schema::gas_plan_settings::dsl::*;
    gas_plan_settings
        .filter(id.eq(user.current_vpm_settings_id))
        .first::<GasPlanSetting>(conn)
}