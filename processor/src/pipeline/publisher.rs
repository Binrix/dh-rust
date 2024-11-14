use std::fs::rename;

use tracing::{error, info};

use super::base::{
    pipeline::{into_next, Pipeline}, 
    pipeline_context::PipelineContext
};

#[derive(Default)]
pub struct Publisher {
    next: Option<Box<dyn Pipeline>>,
}

impl Publisher {
    pub fn new(next: impl Pipeline + 'static) -> Self {
        Self {
            next: into_next(next)
        }
    }

    fn publish(&mut self, file_path: &str) {
        match rename(file_path, "./publish/published_anonymized.json") {
            Ok(_) => info!("File was published"),
            Err(e) => error!("File was not published: {}", e)
        };
    }
}

impl Pipeline for Publisher {
    fn handle(&mut self, context: &mut PipelineContext) {
        self.publish("./file_to_process/anonymized.json");
    }

    fn next(&mut self) -> &mut Option<Box<dyn Pipeline>> {
        &mut self.next
    }
}