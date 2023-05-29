use axum::{body::Body, Extension};
use http::Request;

use crate::{
    controllers::authentication::login_authorized::AuthedUser, errors::AppError,
    repository::mongodb_repo::MongoRepo,
};

async fn validate_house_request(
    req: Request<Body>,
    user: AuthedUser,
    Extension(db): Extension<MongoRepo>,
) -> Result<Request<Body>, AppError> {
    let user_email = user.email;
}
