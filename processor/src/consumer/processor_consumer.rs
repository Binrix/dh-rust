use super::connection_details::connection_details::ConnectionDetails;

use tokio::time::sleep;
use tracing::info;

use std::{
    error::Error, 
    time::Duration
};
use iggy::{
    client::{
        Client, MessageClient, UserClient
    }, 
    clients::client::IggyClient, 
    consumer::Consumer, 
    messages::poll_messages::PollingStrategy, 
    models::messages::{
        PolledMessage, PolledMessages
    }, 
    users::defaults::{
        DEFAULT_ROOT_PASSWORD, DEFAULT_ROOT_USERNAME
    }
};


pub struct ProcessorConsumer {
    client: IggyClient,
    consumer: Consumer,
    connection_details: ConnectionDetails,
    poll_interval: Duration,
}

impl ProcessorConsumer {
    /// Implements the consumer to create a connection to the message broker and consume messages.
    pub async fn new(connection_details: ConnectionDetails, poll_interval: Duration) -> Self {
        let client = IggyClient::default();
        client.connect().await.unwrap();
        client
            .login_user(DEFAULT_ROOT_USERNAME, DEFAULT_ROOT_PASSWORD)
            .await.unwrap();

        Self {
            client: client,
            poll_interval: poll_interval,
            connection_details: connection_details,
            consumer: Consumer::default()
        }
    }

    /// Tries to poll messages from the queue.
    /// ## Arguments
    /// * `offset` - The offset. Used to skip messages in the queue.
    async fn poll_messages(&self, offset: u64) -> PolledMessages {
        self.client
            .poll_messages(
                &self.connection_details.stream_id.try_into().unwrap(), 
                &self.connection_details.topic_id.try_into().unwrap(), 
                Some(self.connection_details.partition_id), 
                &self.consumer, 
                &PollingStrategy::offset(offset), 
                1, false).await.unwrap()
    }

    /// Initates the polling for the specified interval.
    /// ## Arguments
    /// * `callback` - The function to be called if there are any messages.
    pub async fn start_polling(&self, callback: fn(path: &str)) {
        let mut offset= 0;

        loop {
            let polled_messages = self.poll_messages(offset).await;

            if polled_messages.messages.is_empty() {
                info!("No files for processing detected.");
                sleep(self.poll_interval).await;
                continue;
            }

            offset += polled_messages.messages.len() as u64;

            for message in polled_messages.messages {
                let file_path = self.handle_message(&message).unwrap();
                callback(file_path);
            }

            sleep(self.poll_interval).await;
        }
    }

    /// Handles the polled message by parsing the payload to a string.
    /// ## Arguments
    /// * `message` - The polled message that contains the payload.
    fn handle_message<'a>(&self, message: &'a PolledMessage) -> Result<&'a str, Box<dyn Error>> {
        Ok(std::str::from_utf8(&message.payload)?)
    }
}