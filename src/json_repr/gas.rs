use serde::{Deserialize, Serialize};
use capra::common::gas;
use std::convert::TryFrom;
use capra::common::gas::GasError;
use actix_web::{ResponseError, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::dev::HttpResponseBuilder;

// Represents a Gas sent by JSON.
// Difference(s):
// - no `n2` field
// - `max_op_depth` field is included for MOD
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Gas {
    o2: usize,
    he: usize,
    max_op_depth: Option<usize>
}

impl Gas {
    pub fn max_op_depth(&self) -> Option<usize> {
        self.max_op_depth
    }
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

#[derive(thiserror::Error, Debug)]
pub enum ServerGasError {
    #[error(transparent)]
    FractionError(#[from] GasError)
}

impl ResponseError for ServerGasError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .body(self.to_string())
    }
}