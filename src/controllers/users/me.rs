use axum::response::IntoResponse;

use crate::controllers::authentication::login_authorized::User;

pub async fn me(user: User) -> impl IntoResponse {
    format!(
        "Welcome to the protected area :)\nHere's your info:\n{:?}",
        user
    )
}