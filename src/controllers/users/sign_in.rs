use axum::{Extension, extract::Json};
use serde::Deserialize;

use crate::{middleware::auth::AuthState, repository::mongodb_repo::MongoRepo, errors::AppError};

#[derive(Deserialize)]
pub struct SignInPayload {
    email: String,
    password: String,
}

/*pub async fn sign_in(
    Extension(auth_state): Extension<AuthState>,
    Extension(database): Extension<MongoRepo>,
    Json(payload): Json<SignInPayload>
) -> Result<impl axum::response::IntoResponse, AppError> {

    let SignInPayload { email, password } = payload;

    let user = database.get_user(email);

    

}*/
