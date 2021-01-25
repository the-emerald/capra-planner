use sled::{Tree, Db};
use capra_core::deco::Tissue;

#[derive(Copy, Clone, Debug)]
pub struct UserID(u64);

// Value component of the users tree
#[derive(Clone, Debug)]
pub struct User {
    name: String,
    id: UserID,
    tissue: Tissue
}

// Key: username, value: ID
#[derive(Clone, Debug)]
pub struct UsersTree(Tree);

impl UsersTree {
    pub fn open(database: &Db) -> sled::Result<Self> {
        Ok(Self(database.open_tree("users")?))
    }
}
