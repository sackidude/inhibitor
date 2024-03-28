pub type Result<T> = core::result::Result<T, Error>;
#[derive(Debug, PartialEq)]
pub enum Error {
    AskamaTemplateError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "internal web error")
    }
}

impl std::error::Error for Error {}

impl From<askama::Error> for Error {
    fn from(err: askama::Error) -> Self {
        println!("A askama templating error has occured: {}", err);
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
