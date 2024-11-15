use std::{fs::rename, path::PathBuf};

use tracing::{error, info};
use uuid::Uuid;

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

    fn publish(&mut self, file_path: &mut PathBuf, publish_folder: &mut PathBuf, uuid: &Uuid) {
        publish_folder.push(format!("{uuid}.json"));
        file_path.set_file_name(format!("{uuid}.json"));

        match rename(file_path, publish_folder) {
            Ok(_) => info!("File was published"),
            Err(e) => error!("File was not published: {}", e)
        };
    }
}

impl Pipeline for Publisher {
    fn handle(&mut self, context: &mut PipelineContext) {
        if let (Some(file_path), Some(uuid), Some(publish_folder)) = (&mut context.file_path, &mut context.uuid, &mut context.publish_folder) {
            self.publish(file_path, publish_folder, uuid);
        }

    }

    fn next(&mut self) -> &mut Option<Box<dyn Pipeline>> {
        &mut self.next
    }
}