use std::path::PathBuf;

use crate::reader;

pub fn tomls_two_json_array(toml_files: Vec<PathBuf>) -> Vec<String> {
    let mut json_array: Vec<String> = Vec::new();

    for tf in toml_files {
        let toml_content =
            reader::read_content(tf).expect("Error reading file content.");

        let toml_value = toml::from_str::<toml::Value>(&toml_content)
            .expect("Error deserializing toml_content.");

        let json = serde_json::to_string(&toml_value)
            .expect("Error serializing toml_value as a String of JSON.");
        json_array.push(json);
    }

    json_array
}
