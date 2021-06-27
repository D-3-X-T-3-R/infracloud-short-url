use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use crate::configuration_parameters;

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

    let url_code = generate_url_code().unwrap();
    let app_name = "short-url";
    let config_param = configuration_parameters::get_configuration_parameters(app_name);
    let short_url=config_param.local_host.to_string()+":"+&config_param.port+"/"+&url_code;
    
    Ok(UrlDet {
        long_url: long_url,
        short_url: short_url,
    })
}