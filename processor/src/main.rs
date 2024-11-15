use std::time::Duration;

use processor::{
    consumer::{
        connection_details::connection_details::ConnectionDetails, 
        processor_consumer::ProcessorConsumer
    }, 
    pipeline::
    {
        anonymize::Anonymize, base::{
            pipeline::Pipeline, 
            pipeline_context::PipelineContext
        }, 
        open_file::OpenFile, publisher::Publisher
    }
};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let consumer = ProcessorConsumer::new(ConnectionDetails::default(), Duration::from_millis(5000)).await;

    consumer.start_polling(start_pipeline).await;
}

fn start_pipeline(file_path: &str) {
    let file_name = file_path.split("/").last().unwrap();

    info!("Processing for file {} starts", file_name);

    let publisher_pipe = Publisher::default();
    let anonymize_pipe = Anonymize::new(publisher_pipe);
    let mut open_file_pipe = OpenFile::new(anonymize_pipe);

    let mut pipeline_context: PipelineContext<'_> = PipelineContext {
        pipeline_name: "Default",
        file_path: file_path,
        file_name: file_name,
        publish_folder: "./publish/",
        ..PipelineContext::default()
    };

    open_file_pipe.execute(&mut pipeline_context);
}