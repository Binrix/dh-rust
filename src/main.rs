use std::io::BufRead;
use pipeline::{anonymize::Anonymize, open_file::OpenFile};
use serde_json::Value;

mod pipeline;
mod Pipeline;

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

pub fn main() {
    let anonymize = Anonymize::default();
    let mut open_file = OpenFile::new(anonymize);

    let mut pipeline_context: PipelineContext<'_> = PipelineContext {
        pipeline_name: "Default".into(),
        file_name: "example.json".into(),
        ..PipelineContext::default()
    };

    open_file.execute(&mut pipeline_context)
}