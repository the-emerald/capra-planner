use crate::db::dives::DivesTree;
use crate::db::settings::SettingsTree;
use crate::db::users::UsersTree;
use sled::Db;

pub mod dives;
pub mod settings;
pub mod users;

#[derive(Clone, Debug)]
pub struct Database {
    users: UsersTree,
    settings: SettingsTree,
    dives: DivesTree,
}

impl Database {
    pub fn new(database: &Db) -> sled::Result<Self> {
        Ok(
            Self {
                users: UsersTree::open(&database)?,
                settings: SettingsTree::open(&database)?,
                dives: DivesTree::open(&database)?
            }
        )
    }
}
