use std::{fs::File, io::{BufRead, BufReader, BufWriter, Write}};
use serde_json::Value;

// #[derive(Default)]
// pub struct PipelineContext<'a> {
//     pub file_name: &'a str,
//     // pub file_content: BufReader<File>
// }

// impl<'a> Default for PipelineContext<'a> {
//     fn default() -> Self {
//         Self {
//             file_name: Default::default(),
//             // file_content: BufReader
//         } 
//     }
// }

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
fn read(file_name: &str) -> std::io::Result<()> {
    let reader = BufReader::new(File::open(file_name)?);
    let mut writer = BufWriter::new(File::create("anonymized.json".to_string())?);

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

    return Ok(())
}

pub fn main() {
    let _ = read("example.json");    
}