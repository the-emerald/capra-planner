use crate::db::dives::DivesTree;
use crate::db::settings::SettingsTree;
use crate::db::users::UsersTree;
use actix_web::dev::HttpResponseBuilder;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use sled::Db;
use thiserror::Error;

pub mod dives;
pub mod settings;
pub mod users;

#[derive(Clone, Debug)]
pub struct Database {
    database: Db,
    pub(crate) users: UsersTree,
    pub(crate) settings: SettingsTree,
    pub(crate) dives: DivesTree,
}

impl Database {
    pub fn new(database: &Db) -> sled::Result<Self> {
        Ok(Self {
            database: database.clone(),
            users: UsersTree::open(&database)?,
            settings: SettingsTree::open(&database)?,
            dives: DivesTree::open(&database)?,
        })
    }
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("sled: internal database error")]
    Sled(#[from] sled::Error),
    #[error("missing entry detected")]
    MissingEntry,
    #[error("could not serialize/deserialize json")]
    Json(#[from] serde_json::Error),
}

impl ResponseError for DatabaseError {
    fn status_code(&self) -> StatusCode {
        match self {
            DatabaseError::Sled(e) => {
                eprintln!("Serious error: {}", e);
                eprintln!("You should stop using the dive planner immediately and file a bug report here: https://github.com/the-emerald/capra-planner/issues");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            DatabaseError::Json(_) => StatusCode::INTERNAL_SERVER_ERROR,
            DatabaseError::MissingEntry => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).body(self.to_string())
    }
}
