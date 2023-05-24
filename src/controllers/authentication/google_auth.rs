use axum::{extract::State, response::{IntoResponse, Redirect}};
use oauth2::{basic::BasicClient, CsrfToken, Scope};

pub async fn google_auth(State(client): State<BasicClient>) -> impl IntoResponse {
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("profile".to_string()))
        .url();

    Redirect::to(auth_url.as_ref())
}
