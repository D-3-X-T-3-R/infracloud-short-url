use crate::configuration_parameters::ConfigurationParameters;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub struct UrlMapDet {
    pub long_url_map: HashMap<String, String>,
    pub short_url_map: HashMap<String, String>,
}

pub fn read_existing_map(config_param: &ConfigurationParameters) -> UrlMapDet {
    let contents = File::open(config_param.url_map_file_path.to_string()).expect(&format!(
        "Could not read file at : {}",
        config_param.url_map_file_path
    ));
    let mut long_url_map: HashMap<String, String> = HashMap::new();
    let mut short_url_map: HashMap<String, String> = HashMap::new();
    let reader = BufReader::new(contents);
    let mut line_no = 1;
    for line in reader.lines() {
        let record = line.expect(&format!("Could not line: {}", line_no));
        let fields: Vec<&str> = record.split('|').collect();
        long_url_map.insert(fields[0].to_string(), fields[1].to_string());
        short_url_map.insert(fields[1].to_string(), fields[0].to_string());
        line_no += 1;
    }

    UrlMapDet {
        long_url_map: long_url_map,
        short_url_map: short_url_map,
    }
}
