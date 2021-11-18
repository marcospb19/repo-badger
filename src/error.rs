use actix_web::{
    error::{ErrorBadRequest, InternalError},
    http::StatusCode,
};

pub fn anyhow_to_internal_error(err: anyhow::Error) -> InternalError<anyhow::Error> {
    InternalError::new(err, StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn anyhow_to_bad_request(err: anyhow::Error) -> actix_web::Error {
    ErrorBadRequest(err)
}
