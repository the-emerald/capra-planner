use serde::{Deserialize, Serialize};
use capra::common::gas;
use std::convert::TryFrom;
use crate::result::ServerGasError;

// Represents a Gas sent by JSON.
// Difference(s):
// - no `n2` field
// - `max_op_depth` field is included for MOD
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Gas {
    pub o2: usize,
    pub he: usize,
    pub mod_: Option<usize>
}

impl Gas {
    pub fn max_op_depth(&self) -> Option<usize> {
        self.mod_
    }
}

impl From<gas::Gas> for Gas {
    fn from(value: gas::Gas) -> Self {
        Self {
            o2: value.o2(),
            he: value.he(),
            mod_: None
        }
    }
}

impl TryFrom<Gas> for gas::Gas {
    type Error = ServerGasError;

    fn try_from(value: Gas) -> Result<Self, Self::Error> {
        Ok(
            Self::new(
            value.o2,
            value.he,
            100 - value.o2 - value.he
            )?
        )
    }
}