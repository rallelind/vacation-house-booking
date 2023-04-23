use aws_sdk_sqs::Client;
use axum::Extension;

use crate::queue;

pub async fn create_smart_docu(Extension(queue_client): Extension<Client>) {

}