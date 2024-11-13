use std::{error::Error, time::Duration};

use dh::pipeline::{
    base::{
        pipeline::Pipeline, 
        pipeline_context::PipelineContext
    }, 
    anonymize::Anonymize, 
    open_file::OpenFile
};
use iggy::{client::{Client, MessageClient, UserClient}, clients::client::IggyClient, consumer::Consumer, messages::{poll_messages::PollingStrategy, send_messages::Message}, models::messages::PolledMessage, users::defaults::{DEFAULT_ROOT_PASSWORD, DEFAULT_ROOT_USERNAME}};
use tokio::time::sleep;
use tracing::info;

const STREAM_ID: u32 = 1;
const TOPIC_ID: u32 = 1;
const PARTITION_ID: u32 = 1;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let client = IggyClient::default();
    client.connect().await.unwrap();
    client
        .login_user(DEFAULT_ROOT_USERNAME, DEFAULT_ROOT_PASSWORD)
        .await.unwrap();
    consume_messages(&client).await
}

async fn consume_messages(client: &IggyClient) {
    let interval = Duration::from_millis(500);
    info!(
        "Messages will be consumed from stream: {}, topic: {}, partition: {} with interval {} ms.",
        STREAM_ID,
        TOPIC_ID,
        PARTITION_ID,
        interval.as_millis()
    );

    let mut offset = 0;
    let messages_per_batch = 10;
    let consumer = Consumer::default();
    let mut strategy = PollingStrategy::offset(offset);

    loop {
        let polled_messages = client
            .poll_messages(
                &STREAM_ID.try_into().unwrap(),
                &TOPIC_ID.try_into().unwrap(),
                Some(PARTITION_ID),
                &consumer,
                &PollingStrategy::offset(offset),
                messages_per_batch,
                false,
            )
            .await.unwrap();

        if polled_messages.messages.is_empty() {
            info!("No messages found.");
            sleep(interval).await;
            continue;
        }

        offset += polled_messages.messages.len() as u64;
        strategy.set_value(offset);
        for message in polled_messages.messages {
            handle_message(&message).unwrap();
        }
        sleep(interval).await;
    }
}

fn handle_message(message: &PolledMessage) -> Result<(), Box<dyn Error>> {
    // The payload can be of any type as it is a raw byte array. In this case it's a simple string.
    let payload = std::str::from_utf8(&message.payload)?;
    info!(
        "Handling message at offset: {}, payload: {}...",
        message.offset, payload
    );
    Ok(())
}