use crate::db::dives::{DiveID, PlanType};
use crate::db::users::UserID;
use crate::db::{Database, DatabaseError};
use crate::json_repr::dive::JSONDive;
use actix_web::web::{Data, Json};
use actix_web::{post, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use time::OffsetDateTime;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct DatetimeRange(pub OffsetDateTime, pub OffsetDateTime);

#[derive(Clone, Serialize, Deserialize)]
pub struct DiveHistoryInput {
    // filter by types
    exclude_types: Option<Vec<PlanType>>,
    // filter by user
    user: Option<UserID>,
    // filter by time range
    datetime_range: Option<DatetimeRange>,
}

// TODO: Figure out exact API for filtering
#[post("/history/")]
pub(crate) async fn dive_history(
    database: Data<Arc<Database>>,
    json: Json<DiveHistoryInput>,
) -> actix_web::Result<HttpResponse> {
    let plan_types = json.clone().exclude_types.unwrap_or(vec![]);
    let dives = database
        .dives
        .dives_iter()
        // User
        .filter(|x| {
            x.as_ref()
                // If database error, let the Err pass.
                // Otherwise, if filter was not specified, let the Ok pass.
                // If the filter was specified then perform a check.
                .map_or(true, |y| json.user.map_or(true, |z| y.1.user == z))
        })
        // Plan type
        .filter(|x| {
            x.as_ref()
                .map_or(true, |y| plan_types.contains(&y.1.plan_type))
        })
        // Time range
        .filter(|x| {
            x.as_ref().map_or(true, |y| {
                json.datetime_range
                    .map_or(true, |z| z.0 > y.1.timestamp && z.1 < y.1.timestamp)
            })
        })
        // Now map them into JSON Dives
        .map(|x| {
            let d = x?;
            Ok((d.0, d.1.into()))
        })
        .collect::<Result<Vec<(DiveID, JSONDive)>, DatabaseError>>()?;

    Ok(HttpResponse::Ok().json(dives))
}
