use axum::{extract::State, Json};
use oauth2::{basic::BasicClient, CsrfToken, Scope};
use serde_json::{Value, json};

pub async fn google_auth(State(client): State<BasicClient>) -> Json<Value> {
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("profile email".to_string()))
        .url();

    Json(json!(auth_url.as_ref()))
}
