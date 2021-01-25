use crate::db::DatabaseError;
use capra_core::deco::Tissue;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use sled::{Db, Tree};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct UserID(pub u64);

// Value component of the users tree
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: UserID,
    pub tissue: Tissue,
}

// Key: username, value: User
#[derive(Clone, Debug)]
pub struct UsersTree(Tree, Db);

impl UsersTree {
    pub fn open(database: &Db) -> sled::Result<Self> {
        Ok(Self(database.open_tree("users")?, database.clone()))
    }

    pub fn add_user(&self, name: String) -> Result<Option<User>, DatabaseError> {
        // Does this user already exist?
        if let None = self.0.get(&name)? {
            return Ok(None);
        }
        let user = User {
            id: UserID(self.1.generate_id()?),
            tissue: Tissue::default(),
        };

        self.0
            .insert(name, serde_json::to_string(&user)?.as_bytes())?;

        Ok(Some(user))
    }

    pub fn get_user(&self, name: String) -> sled::Result<Option<User>> {
        todo!()
    }
}
