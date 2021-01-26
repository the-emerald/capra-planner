use crate::result::ServerGasError;
use capra_core::common::{gas, Gas};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

// Represents a Gas sent by JSON.
// Difference(s):
// - no `n2` field
// - `max_op_depth` field is included for MOD
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct JSONGas {
    pub o2: usize,
    pub he: usize,
    pub mod_: Option<usize>,
}

impl JSONGas {
    pub fn max_op_depth(&self) -> Option<usize> {
        self.mod_
    }
}

impl From<(gas::Gas, Option<usize>)> for JSONGas {
    fn from(val: (Gas, Option<usize>)) -> Self {
        Self {
            o2: val.0.o2(),
            he: val.0.he(),
            mod_: val.1
        }
    }
}

impl From<gas::Gas> for JSONGas {
    fn from(value: gas::Gas) -> Self {
        Self {
            o2: value.o2(),
            he: value.he(),
            mod_: None,
        }
    }
}

impl TryFrom<JSONGas> for gas::Gas {
    type Error = ServerGasError;

    fn try_from(value: JSONGas) -> Result<Self, Self::Error> {
        Ok(Self::new(value.o2, value.he, 100 - value.o2 - value.he)?)
    }
}
