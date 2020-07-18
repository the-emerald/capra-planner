use crate::db::models;
use crate::simplified::SimplifiedDive;
use diesel::{SqliteConnection, Connection, RunQueryDsl};
use crate::db::models::dive::NewDive;
use chrono::Utc;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::json_repr;
use crate::db::models::gas::{NewGas, Gas};
use crate::db::models::segments::NewSegment;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub enum DiveType {
    PLAN,
    EXECUTE
}

impl DiveType {
    pub fn value(&self) -> i32 {
        match self {
            DiveType::PLAN => { 0 },
            DiveType::EXECUTE => { 1 },
        }
    }
}

pub fn add_dive(
    user: &models::user::User,
    dive: &SimplifiedDive,
    dive_type: &DiveType,
    conn: &SqliteConnection
) -> Result<(), diesel::result::Error> {
    use crate::db::schema::dives::dsl::*;

    conn.transaction::<(), diesel::result::Error, _>(|| {
        let dt = Utc::now().timestamp();
        // Insert dive record
        let new_dive = NewDive {
            user_id: user.id,
            tissue_before_id: user.current_tissue_id,
            timestamp: NaiveDateTime::from_timestamp(dt, 0),
            executed: dive_type.value(),
            zhl_settings_id: user.current_zhl_settings_id,
            vpm_settings_id: user.current_vpm_settings_id,
            general_settings_id: user.current_general_settings_id
        };

        diesel::insert_into(dives)
            .values(new_dive)
            .execute(conn)?;

        let dive_record = dives
            .filter(user_id.eq(&user.id))
            .order(user_id.desc())
            .first::<models::dive::Dive>(conn)?;

        // Insert segments
        for (sg, gs) in &dive.segments {
            add_segment(dive_record.id, sg, gs, conn)?;
        }

        // Insert deco gases
        for dcg in &dive.deco_gases {
            add_gas(Some(dive_record.id), dcg, conn)?;
        }

        Ok(())
    })
}

pub fn add_segment(
    dive_id_: i32,
    segment: &json_repr::dive_segment::DiveSegment,
    gas: &json_repr::gas::Gas,
    conn: &SqliteConnection
) -> Result<(), diesel::result::Error> {
    use crate::db::schema::segments::dsl::*;

    conn.transaction::<(), diesel::result::Error, _>(|| {
        // Do the gas first
        add_gas(None, gas, conn)?;

        let gas_row = {
            use crate::db::schema::gases::dsl::*;
            gases
                .filter(dive_id.is_null())
                .filter(o2.eq(gas.o2 as i32))
                .filter(he.eq(gas.he as i32))
                .filter(max_operating_depth.is_null()) // segment gas cannot have MOD
                .first::<Gas>(conn)?
        };

        // Now the segment
        let new_segment = NewSegment {
            dive_id: dive_id_,
            start_depth: segment.start_depth as i32,
            end_depth: segment.end_depth as i32,
            time: segment.time as i32,
            ascent_rate: segment.ascent_rate as i32,
            descent_rate: segment.descent_rate as i32,
            gas_id: gas_row.id
        };

        diesel::insert_into(segments)
            .values(new_segment)
            .execute(conn)?;

        Ok(())
    })
}

pub fn add_gas(
    dive_id_: Option<i32>,
    gas: &json_repr::gas::Gas,
    conn: &SqliteConnection
) -> Result<(), diesel::result::Error> {
    use crate::db::schema::gases::dsl::*;

    conn.transaction::<(), diesel::result::Error, _>(|| {
        let new_gas = NewGas {
            dive_id: dive_id_,
            o2: gas.o2 as i32,
            he: gas.he as i32,
            max_operating_depth: gas.max_op_depth.and_then(|x| Some(x as i32))
        };

        diesel::insert_into(gases)
            .values(new_gas)
            .execute(conn)?;

        Ok(())
    })
}