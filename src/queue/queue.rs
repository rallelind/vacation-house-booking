use aws_sdk_sqs::{Client, Error};

#[derive(Debug)]
pub struct SQSMessage {
    pub body: String,
    pub group: String,
}

pub async fn send(client: &Client, queue_url: &String, message: &SQSMessage) -> Result<(), Error> {
    println!("Sending message to queue with URL: {}", queue_url);

    let rsp = client
        .send_message()
        .queue_url(queue_url)
        .message_body(&message.body)
        .message_group_id(&message.group)
        // If the queue is FIFO, you need to set .message_deduplication_id
        // or configure the queue for ContentBasedDeduplication.
        .send()
        .await?;

    println!("Send message to the queue: {:#?}", rsp);

    Ok(())
}

pub async fn receive() {

}