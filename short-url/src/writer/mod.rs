use crate::configuration_parameters::ConfigurationParameters;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

pub fn write_to_file(url_map: HashMap<String, String>, config_params: ConfigurationParameters) {
    let mut file = fs::OpenOptions::new()
        .truncate(true)
        .create(true)
        .write(true)
        .open(config_params.url_map_file_path.to_string())
        .unwrap();
    for (key, value) in url_map {
        let writable_data = key + "|" + &value;
        writeln!(file, "{}", writable_data);
    }
}
