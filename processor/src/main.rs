use std::time::Duration;

use processor::{
    consumer::{
        connection_details::connection_details::ConnectionDetails, 
        processor_consumer::ProcessorConsumer
    }, 
    pipeline::builder::pipeline_builder::ProcessorBuilder
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let consumer = ProcessorConsumer::new(ConnectionDetails::default(), Duration::from_millis(5000)).await;

    consumer.start_polling(start_pipeline).await;
}

fn start_pipeline(file_path: &str) {
    let mut pipeline = ProcessorBuilder::new(file_path);
    pipeline.execute_pipeline();
}