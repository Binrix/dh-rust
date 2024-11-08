use std::{default, fs::{read_to_string, File}, io::{BufRead, BufReader, BufWriter, Write}};
use serde_json::Value;

#[derive(Default)]
pub struct PipelineContext<'a> {
    pub file_name: &'a str,
    pub buffer: Option<BufReader<File>>
}

pub trait Pipeline {
    fn execute(&mut self, context: &mut PipelineContext) {
        self.handle(context);

        if let Some(next) = &mut self.next() {
            next.execute(context);
        }
    }

    fn handle(&mut self, context: &mut PipelineContext);
    fn next(&mut self) -> &mut Option<Box<dyn Pipeline>>;
}

pub fn into_next(pipeline: impl Pipeline + Sized + 'static) -> Option<Box<dyn Pipeline>> {
    Some(Box::new(pipeline))
}

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
                            println!("Property {} was updated", &path);
                        }
                    }
                    println!("{}", json);
                }
                let _ = writeln!(writer, "{}", &json);
            });
    }
}



impl Pipeline for Anonymize {
    fn handle(&mut self, context: &mut PipelineContext) {
        if let Some(ref mut reader) = context.buffer {
            self.read(reader);
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Pipeline>> {
        &mut self.next
    }
}

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
        let file = File::open(context.file_name).unwrap();
        context.buffer = Some(BufReader::new(file));
    }

    fn next(&mut self) -> &mut Option<Box<dyn Pipeline>> {
        &mut self.next
    }
}

fn is_event_sensitive(json: &serde_json::Value) -> bool {
    json.get("sensitive")
        .and_then(|sensitive: &Value| sensitive.as_bool())
        .unwrap_or(false)
}

fn anonymize_property(json: &mut serde_json::Value, path: &str) -> bool {
    let keys: Vec<&str> = path.split(".").skip(1).collect();
    let mut current_json: &mut Value = json;

    for (index, key) in keys.iter().enumerate() {
        if index == keys.len() - 1 {
            if let Some(obj) = current_json.as_object_mut() {
                obj.insert(key.to_string(), Value::String("Anonymize".to_string()));
                return true;
            } 
        } else {
            current_json = match current_json.get_mut(*key) {
                Some(value) => value,
                None => return false
            }
        }
    }

    return false;
}

/// Reads the content of a file line by line. Replaces sensitive data.


pub fn main() {
    let mut openfile = OpenFile::default();
    let mut anonymize = Anonymize::new(openfile);

    let mut pipeline_context = PipelineContext {
        file_name: "example.json".into(),
        ..PipelineContext::default()
    };

    openfile.execute(&mut pipeline_context)

    // let _ = read("example.json");    
}