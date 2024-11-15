use std::{
    error::Error, ffi::OsString, fs, io
};
use ingress::producer::{
    connection_details::connection_details::ConnectionDetails, 
    ingress_producer::IngressProducer
};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let producer = IngressProducer::new(ConnectionDetails::default()).await;
    let mut word = String::new();

    loop {
        println!("Type something a single character to scan the directory");

        io::stdin()
            .read_line(&mut word)
            .expect("Failed to read line");

        word = word.trim().to_string();

        if word.len() != 1 { break; }

        for new_file in scan_dir("./files_to_process") {
            producer.send_file_for_processing_detected(new_file.to_str().unwrap()).await;
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