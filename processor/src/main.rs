use std::{error::Error, time::Duration};

use processor::pipeline::{
    anonymize::Anonymize, base::{
        pipeline::Pipeline, 
        pipeline_context::PipelineContext
    }, open_file::OpenFile, publisher::Publisher
};
use iggy::{
    client::{
        Client, MessageClient, UserClient
    }, 
    clients::client::IggyClient, 
    consumer::Consumer, 
    messages::poll_messages::PollingStrategy, 
    models::messages::PolledMessage, 
    users::defaults::{
        DEFAULT_ROOT_PASSWORD, DEFAULT_ROOT_USERNAME
    }
};
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
    let interval = Duration::from_millis(5000);
    info!("Processor starts listening");

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
            info!("No files for processing detected.");
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
    let payload = std::str::from_utf8(&message.payload)?;
    info!("File for processing detected: {}", payload);

    let publisher_pipe = Publisher::default();
    let anonymize_pipe = Anonymize::new(publisher_pipe);
    let mut open_file_pipe = OpenFile::new(anonymize_pipe);

    let mut pipeline_context: PipelineContext<'_> = PipelineContext {
        pipeline_name: "Default".into(),
        file_path: payload.into(),
        ..PipelineContext::default()
    };

    open_file_pipe.execute(&mut pipeline_context);

    Ok(())
}