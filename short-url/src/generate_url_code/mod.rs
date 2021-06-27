use crate::configuration_parameters;
use crate::reader::read_existing_map;
use crate::writer::write_to_file;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UrlDet {
    pub long_url: String,
    pub short_url: String,
}

pub fn generate_url_code() -> Result<String, String> {
    let url_code = nanoid!(8);
    Ok(url_code)
}

pub fn generate_shorten_url(long_url: String) -> Result<UrlDet, String> {
    let app_name = "short-url";
    let config_param = configuration_parameters::get_configuration_parameters(app_name);
    let mut existing_map = read_existing_map(&config_param);
    let local_host = config_param.local_host.to_string() + ":" + &config_param.port + "/";
    let short_url = if existing_map.long_url_map.contains_key(&long_url) {
        let url_code = existing_map
            .long_url_map
            .get(&long_url)
            .unwrap()
            .to_string();
        let shorten = local_host + &url_code;
        shorten
    } else {
        let url_code = generate_url_code().unwrap();
        let shorten = local_host + &url_code;
        existing_map
            .long_url_map
            .insert(long_url.to_string(), url_code.to_string());
        existing_map
            .short_url_map
            .insert(url_code.to_string(), long_url.to_string());
        shorten
    };
    write_to_file(existing_map.long_url_map, config_param);

    Ok(UrlDet {
        long_url: long_url,
        short_url: short_url,
    })
}
