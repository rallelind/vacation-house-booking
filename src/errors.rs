use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    InvalidToken,
    WrongCredential,
    MissingCredential,
    TokenCreation,
    InternalServerError,
    UserDoesNotExist,
    UserAlreadyExist,
    PasswordMismatch,
    MissingUserIdForProvidedUsers,
    BookingAlreadyExists,
    UserIsNotPartOfHouse
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an internal server error happened"
            ),
            Self::InvalidToken => (StatusCode::BAD_REQUEST, "invalid token"),
            Self::WrongCredential => (StatusCode::BAD_REQUEST, "wrong credentials provided"),
            Self::MissingCredential => (StatusCode::BAD_REQUEST, "missing credentials"),
            Self::PasswordMismatch => (StatusCode::BAD_REQUEST, "the passwords did not match"),
            Self::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "error creating token"),
            Self::UserAlreadyExist => (StatusCode::UNAUTHORIZED, "user already exists"),
            Self::UserDoesNotExist => (StatusCode::UNAUTHORIZED, "user does not exist"),
            Self::MissingUserIdForProvidedUsers => (StatusCode::UNPROCESSABLE_ENTITY, "please provide user id's for users in payload"),
            Self::BookingAlreadyExists => (StatusCode::BAD_REQUEST, "there already is a booking made"),
            Self::UserIsNotPartOfHouse => (StatusCode::UNAUTHORIZED, "you are not part of this house")
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}