use crate::db::DatabaseError;
use capra_core::deco::Tissue;
use serde::{Deserialize, Serialize};
use sled::{Db, Tree};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct UserID(pub u64);

// Value component of the users tree
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub tissue: Tissue,
}

// Key: username, value: User
#[derive(Clone, Debug)]
pub struct UsersTree(Tree, Db);

impl UsersTree {
    pub fn open(database: &Db) -> sled::Result<Self> {
        Ok(Self(database.open_tree("users")?, database.clone()))
    }

    pub fn add_user(&self, name: String) -> Result<Option<UserID>, DatabaseError> {
        // Does this user already exist?
        if !self.0.iter().try_fold(false, |a, b| {
            let u: User = serde_json::from_slice(&*b?.1)?;
            Ok::<bool, DatabaseError>(a | (u.name == name))
        })? {
            return Ok(None);
        }

        let user = User {
            name,
            tissue: Tissue::default(),
        };

        let id = UserID(self.1.generate_id()?);

        self.0
            .insert(serde_json::to_vec(&id)?, serde_json::to_vec(&user)?)?;

        Ok(Some(id))
    }

    pub fn get_user(&self, id: &UserID) -> Result<Option<User>, DatabaseError> {
        let k = serde_json::to_vec(&id)?;
        if !self.0.contains_key(&k)? {
            return Ok(None);
        }

        Ok(Some(serde_json::from_slice(&*self.0.get(&k)?.unwrap())?))
    }

    pub fn get_all_users(&self) -> Result<Vec<(UserID, User)>, DatabaseError> {
        self.0
            .iter()
            .map(|x| {
                let y = x?;
                Ok((
                    serde_json::from_slice(&*y.0)?,
                    serde_json::from_slice(&*y.1)?,
                ))
            })
            .collect()
    }
}
