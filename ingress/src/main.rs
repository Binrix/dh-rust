use std::{error::Error, ffi::OsString, fs::{self, DirEntry}, io, str::FromStr};

use iggy::{
    client::{Client, MessageClient, StreamClient, TopicClient, UserClient}, 
    clients::client::IggyClient, compression::compression_algorithm::CompressionAlgorithm, messages::send_messages::{Message, Partitioning}, users::defaults::{DEFAULT_ROOT_PASSWORD, DEFAULT_ROOT_USERNAME}, utils::expiry::IggyExpiry
};
use tracing::{error, info, warn};

const STREAM_ID: u32 = 1;
const TOPIC_ID: u32 = 1;
const PARTITION_ID: u32 = 1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let mut word = String::new();
    let client = IggyClient::default();
    
    client.connect().await?;
    client
        .login_user(DEFAULT_ROOT_USERNAME, DEFAULT_ROOT_PASSWORD)
        .await?;

    init_system(&client).await;

    loop {
        println!("Type something a single character to scan the directory");

        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");

        word = word.trim().to_string();

        if word.len() != 1 { break; }

        for new_file in scan_dir("./files_to_process") {
            send_file_for_processing_detected(&client, new_file.to_str().unwrap()).await;
        }

        word.clear();
    }

    Ok(())
}

/// Scans the directory. 
/// 
/// All scanned files will be added to a list. It will skip the files contained in the list.
/// # Arguments
/// * `path` - The path to the directory.
fn scan_dir(path: &str) -> impl Iterator<Item = OsString> {
    info!("Scanning directory {path}");

    match fs::read_dir(path) {
        Ok(paths) => paths
            .filter_map(|path| Some(path.unwrap().path().into_os_string())),
        Err(e) => { 
            error!("Unable to read directory: {path}, with the following error: {}", &e);
            panic!()
        },
    }
}

async fn init_system(client: &IggyClient) {
    match client.create_stream("test-stream", Some(STREAM_ID)).await {
        Ok(_) => info!("Stream was created"),
        Err(_) => warn!("Stream already exists and will not be created again.")
    }

    match client
        .create_topic(
            &STREAM_ID.try_into().unwrap(), 
            "test-stream", 
            1, 
            CompressionAlgorithm::default(), 
            None, 
            Some(TOPIC_ID), 
            IggyExpiry::NeverExpire, 
            None.into())
            .await 
        {
            Ok(_) => info!("Topic was created"),
            Err(_) => warn!("Topic already exists and will not be created again.")
        }

}

/// Sends a FileForProcessingDetected message.
/// # Arguments
/// * `client` - The client for sending the message.
/// * `path_to_file` - The path for the file which was detected
async fn send_file_for_processing_detected(client: &IggyClient, path_to_file: &str) {
    let partitioning = Partitioning::partition_id(PARTITION_ID);

    let message = Message::from_str(path_to_file).unwrap();

    client.send_messages(&STREAM_ID.try_into().unwrap(), &TOPIC_ID.try_into().unwrap(), &partitioning, &mut [message]).await.unwrap();
    info!("Sent FileForProcessingDetected message.");
}