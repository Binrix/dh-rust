use std::{borrow::Borrow, error::Error, ffi::{OsStr, OsString}, fs, str::FromStr, time::Duration};

use iggy::{
    client::{Client, MessageClient, StreamClient, TopicClient, UserClient}, clients::client::IggyClient, compression::compression_algorithm::CompressionAlgorithm, messages::send_messages::{Message, Partitioning}, users::defaults::{DEFAULT_ROOT_PASSWORD, DEFAULT_ROOT_USERNAME}, utils::expiry::IggyExpiry
};
use tokio::{time::sleep};
use tracing::{info, warn};

const STREAM_ID: u32 = 1;
const TOPIC_ID: u32 = 1;
const PARTITION_ID: u32 = 1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    // let client = IggyClient::default();
    
    // client.connect().await?;
    // client
    //     .login_user(DEFAULT_ROOT_USERNAME, DEFAULT_ROOT_PASSWORD)
    //     .await?;

    // init_system(&client).await;

    // produce_message(&client).await;

    scan_dir("./files_to_process").await;

    Ok(())
}

async fn scan_dir(path: &str) {
    info!("Scanning directory {path}");

    let interval = Duration::from_millis(5000);
    let mut detected_files = Vec::<OsString>::new(); 

    loop {
        let paths = fs::read_dir(path).unwrap();

        let new_files: Vec::<OsString> = paths
            .filter_map(|path| Some(path.unwrap().file_name().to_os_string()))
            .filter(|path| !detected_files.contains(path))
            .collect();

        for file in new_files {
            info!("File {:?} for processing detected", file);
            detected_files.push(file);
        }

        // paths
        //     .filter_map(|path| Some(path.unwrap().file_name().to_os_string()))
        //     .filter(|path| !detected_files.contains(&path))
        //     .for_each(|path| {
        //         info!("File {:?} for processing detected.", path);
        //         detected_files.push(path.clone());
        //         detected_files.push(path.to_os_string());
        //     });

        // for path in paths {
        //     let file_name = path.unwrap().file_name();

        //     info!("File {:?} was found.", file_name);
        //     detected_files.push(file_name);
        // }

        // info!("test");

        let length = detected_files.len();
        info!("Number of detected files: {}", length);

        sleep(interval).await;
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

async fn send_file_for_processing_detected(client: &IggyClient, path_to_file: &str) {
    let interval = Duration::from_millis(2000);

    info!("Messages will be sent to stream: {}, topic: {}, partition: {} with interval {} ms.", STREAM_ID, TOPIC_ID, PARTITION_ID, interval.as_millis());

    let mut current_id = 0;
    let messages_per_batch = 3;
    let partitioning = Partitioning::partition_id(PARTITION_ID);

    // loop {
    //     let mut messages = Vec::new();

    //     for _ in 0..messages_per_batch {
    //         current_id += 1;
    //         let payload = format!("message-{current_id}");
    //         let message = Message::from_str(&payload).unwrap();
    //         messages.push(message);
    //     }
    //     client.send_messages(&STREAM_ID.try_into().unwrap(), &TOPIC_ID.try_into().unwrap(), &partitioning, &mut messages).await.unwrap();
    //     info!("Sent {messages_per_batch} message(s).");
    //     sleep(interval).await;
    // }

}