use std::str::FromStr;
use iggy::{
    client::{
        Client, 
        MessageClient, 
        StreamClient, 
        TopicClient, 
        UserClient
    }, 
    clients::client::IggyClient, 
    compression::compression_algorithm::CompressionAlgorithm, 
    messages::send_messages::{
        Message, Partitioning
    }, 
    users::defaults::{
        DEFAULT_ROOT_PASSWORD, DEFAULT_ROOT_USERNAME
    }, 
    utils::expiry::IggyExpiry
};
use tracing::{info, warn};

use super::connection_details::connection_details::ConnectionDetails;

pub struct IngressProducer {
    client: IggyClient,
    connection_details: ConnectionDetails
}

impl IngressProducer {
    pub async fn new(connection_details: ConnectionDetails) -> Self {
        let client = IggyClient::default();
        let _ = client.connect().await;
        let _ = client
            .login_user(DEFAULT_ROOT_USERNAME, DEFAULT_ROOT_PASSWORD)
            .await;

        match client.create_stream("test-stream", Some(connection_details.stream_id)).await {
            Ok(_) => info!("Stream was created"),
            Err(_) => warn!("Stream already exists and will not be created again.")
        }

        match client
            .create_topic(
                &connection_details.stream_id.try_into().unwrap(), 
                "test-stream", 
                1, 
                CompressionAlgorithm::default(), 
                None, 
                connection_details.topic_id.try_into().unwrap(), 
                IggyExpiry::NeverExpire, 
                None.into())
                .await 
            {
                Ok(_) => info!("Topic was created"),
                Err(_) => warn!("Topic already exists and will not be created again.")
            }

        Self {
            client,
            connection_details
        }
    }

    /// Sends a FileForProcessingDetected message.
    /// # Arguments
    /// * `client` - The client for sending the message.
    /// * `path_to_file` - The path for the file which was detected
    pub async fn send_file_for_processing_detected(&self, path_to_file: &str) {
        let partitioning = Partitioning::partition_id(self.connection_details.partition_id);
        let message = Message::from_str(path_to_file).unwrap();
    
        self.client.send_messages(
            &self.connection_details.stream_id.try_into().unwrap(), 
            &self.connection_details.topic_id.try_into().unwrap(), 
            &partitioning, &mut [message])
            .await.unwrap();
        info!("Sent FileForProcessingDetected message.");
    }
}