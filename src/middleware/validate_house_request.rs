use axum::{
    extract::Path, http::Request, middleware::Next, response::Response, Extension,
};

use crate::{
    errors::AppError,
    repository::mongodb_repo::MongoRepo,
};

pub async fn validate_house_request<B>(
    Path((house_id, user_id)): Path<(String, String)>,
    Extension(db): Extension<MongoRepo>,
    req: Request<B>,
    next: Next<B>
) -> Result<Response, AppError> {
    let found_house = db.user_part_of_house(house_id, user_id);

    if found_house {
        Ok(next.run(req).await)
    } else {
        Err(AppError::UserIsNotPartOfHouse)
    }
}
