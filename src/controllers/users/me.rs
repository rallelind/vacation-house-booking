use axum::response::IntoResponse;

use crate::controllers::authentication::login_authorized::User;

pub async fn me(user: Option<User>) -> impl IntoResponse {
    match user {
        Some(u) => format!(
            "Hey {}! You're logged in!\nYou may now access `/protected`.\nLog out with `/logout`.",
            u.name
        ),
        None => "You're not logged in.\nVisit `/auth/discord` to do so.".to_string(),
    }
}