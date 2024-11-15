use std::{fs::File, io::BufReader, path::PathBuf};

use uuid::Uuid;

#[derive(Default)]
pub struct PipelineContext<'a> {
    pub pipeline_name: &'a str,
    pub file_path: Option<PathBuf>,
    pub file_name:  &'a str,
    pub uuid: Option<Uuid>,
    pub publish_folder: Option<PathBuf>,
    pub buffer: Option<BufReader<File>>
}