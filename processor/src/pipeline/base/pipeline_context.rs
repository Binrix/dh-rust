use uuid::Uuid;
use std::{
    fs::File, 
    io::BufReader, 
    path::PathBuf
};


#[derive(Default)]
pub struct PipelineContext<'a> {
    pub pipeline_name: &'a str,
    pub file_path: PathBuf,
    pub file_name:  &'a str,
    pub uuid: Uuid,
    pub publish_folder: PathBuf,
    pub buffer: Option<BufReader<File>>
}