use std::path::PathBuf;

use uuid::Uuid;

use crate::pipeline::{
    anonymize::Anonymize, base::{pipeline::Pipeline, pipeline_context::PipelineContext}, open_file::OpenFile, publisher::Publisher
};

pub struct ProcessorBuilder<'a> {
    pipeline: Box<dyn Pipeline + 'static>,
    pipeline_context: PipelineContext<'a>
}

impl<'a> ProcessorBuilder<'a> {
    pub fn new(file_path: &'a str) -> Self {
        let publisher_pipe = Publisher::default();
        let anonymize_pipe = Anonymize::new(publisher_pipe);
        let open_file_pipe = OpenFile::new(anonymize_pipe);
        let file_name = file_path.split("/").last().unwrap();

        let pipeline_context: PipelineContext<'a> = PipelineContext {
            pipeline_name: "Default",
            file_path: PathBuf::from(file_path),
            uuid: Uuid::new_v4(),
            file_name: file_name,
            // file_name_after_processing: "anonymized.json",
            publish_folder: PathBuf::from("./publish/"),
            ..PipelineContext::default()
        };

        Self {
            pipeline: Box::new(open_file_pipe),
            pipeline_context
        }
    }

    /// Executes the pipeline with the specified pipeline context.
    pub fn execute_pipeline(&mut self) {
        self.pipeline.execute(&mut self.pipeline_context);
    }
}