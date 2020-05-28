use serde::{Deserialize, Serialize};
use capra::common::gas;
use std::convert::TryFrom;
use capra::common::gas::GasError;

#[derive(Serialize, Deserialize)]
pub(crate) struct Gas {
    o2: usize,
    he: usize,
    max_op_depth: Option<usize>
}

impl From<gas::Gas> for Gas {
    fn from(value: gas::Gas) -> Self {
        Self {
            o2: value.o2(),
            he: value.he(),
            max_op_depth: None
        }
    }
}

impl TryFrom<Gas> for gas::Gas {
    type Error = GasError;

    fn try_from(value: Gas) -> Result<Self, Self::Error> {
        Self::new(
            value.o2,
            value.he,
            100 - value.o2 - value.he
        )
    }
}