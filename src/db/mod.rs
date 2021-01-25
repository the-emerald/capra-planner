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
    #[error("internal database error")]
    Sled(#[from] sled::Error),
    #[error("could not serialize/deserialize json")]
    Json(#[from] serde_json::Error),
}

impl ResponseError for DatabaseError {
    fn status_code(&self) -> StatusCode {
        match self {
            DatabaseError::Sled(e) => {
                eprintln!("Serious error: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            DatabaseError::Json(e) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).body(self.to_string())
    }
}
