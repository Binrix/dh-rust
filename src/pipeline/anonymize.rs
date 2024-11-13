use std::{fs::File, io::{BufRead, BufReader, BufWriter, Write}};
use serde_json::Value;

use super::{
    base::{
        pipeline::{into_next, Pipeline}, 
        pipeline_context::PipelineContext
    }, 
    helper::helper::{anonymize_property, is_event_sensitive}
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
    fn read(&mut self, reader: &mut BufReader<File>) {
        let mut writer = BufWriter::new(File::create("anonymized.json").unwrap());
        // let reader = &context.file_reader; 

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
        println!("work on anonymization");
        if let Some(ref mut reader) = context.buffer {
            self.read(reader);
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Pipeline>> {
        &mut self.next
    }
}
