use serde_json::Value;

/// Replaces a value by "Anonymize". 
/// The JSON object contains paths to the property which will be anonymized.
/// 
/// ## Arguments
/// * `json` - The JSON parsed from the current line
/// * `path` - The path to the property
pub fn anonymize_property(json: &mut serde_json::Value, path: &str) -> bool {
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

    false
}

/// Checks the property `sensitive` to see wether anonymization is required.
/// 
/// # Arguments
/// 
/// * `json` - The JSON parsed from the current line
pub fn is_event_sensitive(json: &serde_json::Value) -> bool {
    json.get("sensitive")
        .and_then(|sensitive: &Value| sensitive.as_bool())
        .unwrap_or(false)
}