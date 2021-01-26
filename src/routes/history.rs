use crate::db::dives::{Dive, DiveID, PlanType};
use crate::db::users::UserID;
use crate::db::{Database, DatabaseError};
use actix_web::web::{Data, Json};
use actix_web::{post, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Serialize, Deserialize)]
pub struct DiveHistoryInput {
    // filter by types
    exclude_types: Option<Vec<PlanType>>,
    // filter by user
    user: Option<UserID>,
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
        .filter(|x| {
            x.as_ref()
                // If database error, let the Err pass.
                // Otherwise, if filter was not specified, let the Ok pass.
                // If the filter was specified then perform a check.
                .map_or(true, |y| json.user.map_or(true, |z| y.1.user == z))
        })
        .filter(|x| {
            x.as_ref()
                .map_or(true, |y| plan_types.contains(&y.1.plan_type))
        })
        .collect::<Result<Vec<(DiveID, Dive)>, DatabaseError>>()?;

    Ok(HttpResponse::Ok().json(dives))
}
