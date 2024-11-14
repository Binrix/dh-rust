use std::{fs::File, io::BufReader};

use super::base::{
    pipeline::{into_next, Pipeline}, 
    pipeline_context::PipelineContext
};

#[derive(Default)]
pub struct OpenFile {
    next: Option<Box<dyn Pipeline>>
}

impl OpenFile {
    pub fn new(next: impl Pipeline + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}

impl Pipeline for OpenFile {
    fn handle(&mut self, context: &mut PipelineContext) {
        println!("Open file: {}", context.file_path);
        match File::open(context.file_path) {
            Ok(stream) => {
                context.buffer = Some(BufReader::new(stream));
            },
            Err(e) => println!("Error reading file {}", e)
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Pipeline>> {
        &mut self.next
    }
}       