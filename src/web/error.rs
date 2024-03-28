use crate::api;

pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug, PartialEq)]
pub enum Error {
    AskamaTemplateError,
    FormDeserializingError,
    DatabaseError,
    ApiError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "internal web error")
    }
}

impl std::error::Error for Error {}

impl From<askama::Error> for Error {
    fn from(err: askama::Error) -> Self {
        println!(
            "web::error::Error.into_response with askama templating error: {}",
            err
        );
        Error::AskamaTemplateError
    }
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("web::error into_response: {:?}", self);

        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "An unexpected internal error has occured.",
        )
            .into_response()
    }
}

impl From<serde_urlencoded::de::Error> for Error {
    fn from(err: serde_urlencoded::de::Error) -> Self {
        println!(
            "web::error::Error.into_response with serde_urlencoded deserializing error: {}",
            err
        );
        Error::FormDeserializingError
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        println!("web::error::Error.into_response with sqlx error: {}", err);
        Error::DatabaseError
    }
}

impl From<api::error::Error> for Error {
    fn from(err: api::error::Error) -> Self {
        println!(
            "web::error:Error.into_response with api error: {:?}, {}",
            err, err
        );
        Error::ApiError
    }
}
