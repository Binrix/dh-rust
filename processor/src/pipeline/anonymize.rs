use serde_json::Value;
use uuid::Uuid;

use std::{
    fs::File, 
    io::{
        BufRead, BufReader, BufWriter, Write
    }, 
    path::PathBuf
};
use super::{
    base::{
        pipeline::{
            into_next, Pipeline
        }, 
        pipeline_context::PipelineContext
    }, 
    helper::helper::{
        anonymize_property, is_event_sensitive
    }
};

#[derive(Default)]
pub struct Anonymize {
    next: Option<Box<dyn Pipeline>>,
}

impl Anonymize {
    pub fn new(next: impl Pipeline + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
    fn read(&mut self, reader: &mut BufReader<File>, file_path: &mut PathBuf, uuid: &mut Uuid) {
        file_path.set_file_name(format!("{uuid}.json"));
        let mut writer = BufWriter::new(File::create(file_path).unwrap());

        reader.lines()
            .filter_map(Result::ok)
            .filter_map(|line: String| serde_json::from_str::<Value>(&line).ok())
            .for_each(|mut json: Value| {
                if is_event_sensitive(&json) {
                    let paths_cloned: Vec<String> = json
                        .get("paths")
                        .and_then(|v: &Value| v.as_array())
                        .map(|paths: &Vec<Value>| {
                            paths
                                .iter()
                                .filter_map(|path: &Value| path.as_str().map(|s| s.to_string()))
                                .collect()
                        })
                        .unwrap_or_default();
    
                    for path in paths_cloned {
                        if anonymize_property(&mut json, &path) {
                            println!("Property {} was anonymized", &path);
                        }
                    }
                }
                let _ = writeln!(writer, "{}", &json);
        });
    }
}

impl Pipeline for Anonymize {
    fn handle(&mut self, context: &mut PipelineContext) {
        println!("Anonymize content...");
        
        if let (Some(reader), Some(file_path), Some(uuid)) = (&mut context.buffer, &mut context.file_path, &mut context.uuid) {
            self.read(reader, file_path, uuid);
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Pipeline>> {
        &mut self.next
    }
}
