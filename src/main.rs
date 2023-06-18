use async_mongodb_session::MongodbSessionStore;
use std::env::var;
use tower::ServiceBuilder;

use axum::{
    middleware::from_fn,
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use tower_http::cors::{Any, CorsLayer};

mod controllers;
mod errors;
mod middleware;
mod models;
mod repository;

use controllers::{
    authentication::{
        google_auth::google_auth, login_authorized::login_authorized, logout::logout,
    },
    family::{create_family::create_family, get_family::get_family},
    house::{create_booking::create_booking, create_house::create_house, get_house::get_house},
    invitations::send_invitation::send_invitation,
    users::me::me,
};
use repository::mongodb_repo::MongoRepo;

use middleware::{
    auth::{oauth_client, AuthState},
    validate_house_request::validate_house_request,
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mongo_connection_string =
        var("MONGO_CONNECTION_STRING").expect("failed to read mongo connection string");

    let store = MongodbSessionStore::new(
        mongo_connection_string.as_str(),
        "house_booking",
        "sessions",
    )
    .await
    .unwrap();
    let client = oauth_client();
    let db = MongoRepo::init();

    let cors = CorsLayer::very_permissive();

    let auth_state = AuthState { store, client };

    let user_routes = Router::new()
        .route("/:houseId/:userId", get(get_house))
        .route("/:houseId/:userId/booking", post(create_booking))
        .layer(from_fn(validate_house_request))
        .route("/", post(create_house));

    let family_routes = Router::new()
        .route("/", post(create_family))
        .route("/", get(get_family));

    let app = Router::new()
        .route("/auth/logout", get(logout))
        .route("/auth/google", get(google_auth))
        .route("/auth/authorized", get(login_authorized))
        .route("/users/invitation", post(send_invitation))
        .route("/users/me", get(me))
        .nest("/house", user_routes)
        .nest("/family", family_routes)
        .layer(
            ServiceBuilder::new()
                .layer(cors)
                .layer(Extension(db))
                .into_inner(),
        )
        .with_state(auth_state);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("starting server on port: {}", addr.port());

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}
