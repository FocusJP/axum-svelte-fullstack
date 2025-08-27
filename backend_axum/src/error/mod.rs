use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    // External errors
    Bb8(#[from] bb8::RunError<tokio_postgres::Error>),
    DotEnv(#[from] dotenv::Error),
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    JsonWebToken(#[from] jsonwebtoken::errors::Error),
    NativeTls(#[from] native_tls::Error),
    ParseInt(#[from] std::num::ParseIntError),
    Reqwest(#[from] reqwest::Error),
    StdEnvVar(#[from] std::env::VarError),
    StdIo(#[from] std::io::Error),
    TokioPostgres(#[from] tokio_postgres::Error),
    TracingEnv(#[from] tracing_subscriber::filter::FromEnvError),
    TypedHeaderRejection(#[from] axum_extra::typed_header::TypedHeaderRejection),

    // Internal errors
    BadRequest(String),
    Forbidden(String),
    Unauthorized,
    UnprocessableEntity(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!(app_error=?self, "AppError");

        let response_tuple: (StatusCode, String) = match self {
            AppError::Bb8(_)
            | AppError::InvalidHeaderValue(_)
            | AppError::JsonWebToken(_)
            | AppError::NativeTls(_)
            | AppError::ParseInt(_)
            | AppError::StdEnvVar(_)
            | AppError::StdIo(_)
            | AppError::TokioPostgres(_)
            | AppError::TracingEnv(_)
            | AppError::DotEnv(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".into(),
            ),

            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".into()),

            AppError::TypedHeaderRejection(msg) => {
                (StatusCode::BAD_REQUEST, format!("Header error: {msg}"))
            }

            AppError::Reqwest(_) => (StatusCode::BAD_GATEWAY, "Bad Gateway".into()),

            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, format!("Bad Request: {msg}")),

            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, format!("Forbidden: {msg}")),

            AppError::UnprocessableEntity(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("Unprocessable entity: {msg}"),
            ),
        };

        response_tuple.into_response()
    }
}
