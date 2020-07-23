use capra::common::gas::GasError;
use capra::common::dive_segment::DiveSegmentError;
use actix_web::{ResponseError, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::dev::HttpResponseBuilder;

#[derive(thiserror::Error, Debug)]
pub enum ZHLSettingError {
    #[error("could not convert string to zhl subtype")]
    ConversionError
}

impl ResponseError for ZHLSettingError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .body(self.to_string())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ServerDivePlanningError {
    #[error(transparent)]
    Gas(#[from] ServerGasError),
    #[error(transparent)]
    DiveSegment(#[from] ServerDiveSegmentError)
}

impl ResponseError for ServerDivePlanningError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .body(self.to_string())
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

#[derive(thiserror::Error, Debug)]
pub enum ServerDiveSegmentError {
    #[error(transparent)]
    IncorrectSegmentTypeError(#[from] DiveSegmentError)
}

impl ResponseError for ServerDiveSegmentError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .body(self.to_string())
    }
}