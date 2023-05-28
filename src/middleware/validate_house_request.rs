use axum::{body::Body, Extension};
use http::Request;

use crate::{errors::AppError, repository::mongodb_repo::MongoRepo, controllers::authentication::login_authorized::AuthedUser};



async fn validate_house_request(req: Request<Body>, user: AuthedUser, Extension(db): Extension<MongoRepo>) -> Result<Request<Body>, AppError> {

    let user_email = user.email;

    

}