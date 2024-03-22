use core::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    RequestParsingError,
    DatabaseQueryError,
    AskamaTemplatingError
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("{self:?}");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An unexpected error has occured.")
    }
}
