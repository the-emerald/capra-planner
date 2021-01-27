use crate::result::ServerGasError;
use capra_core::common::{gas, Gas};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

// Represents a Gas sent by JSON.
// Difference(s):
// - no `n2` field
// - `max_op_depth` field is included for MOD
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct JSONGas {
    pub o2: usize,
    pub he: usize,
    pub max_operating_depth: Option<usize>,
}

impl JSONGas {
    pub fn max_op_depth(&self) -> Option<usize> {
        self.max_operating_depth
    }
}

impl From<(gas::Gas, Option<usize>)> for JSONGas {
    fn from(val: (Gas, Option<usize>)) -> Self {
        Self {
            o2: val.0.o2(),
            he: val.0.he(),
            max_operating_depth: val.1,
        }
    }
}

impl From<gas::Gas> for JSONGas {
    fn from(value: gas::Gas) -> Self {
        Self {
            o2: value.o2(),
            he: value.he(),
            max_operating_depth: None,
        }
    }
}

impl TryFrom<JSONGas> for gas::Gas {
    type Error = ServerGasError;

    fn try_from(value: JSONGas) -> Result<Self, Self::Error> {
        Ok(Self::new(value.o2, value.he, 100 - value.o2 - value.he)?)
    }
}
