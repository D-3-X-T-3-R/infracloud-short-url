use clap::{App, Arg};

pub fn get_configuration_parameters(app_name: &str) -> ConfigurationParameters {
    let matches = get_eligible_arguments_for_app(app_name);
    ConfigurationParameters::new_from_matches(matches)
}

#[derive(Debug)]
pub struct ConfigurationParameters {
    pub local_host: String,
    pub port: String,
    pub url_map_file_path: String,
}

impl ConfigurationParameters {
    fn new_from_matches(matches: clap::ArgMatches) -> ConfigurationParameters {
        let local_host = matches
            .value_of("local_host")
            .expect("Error getting `local host` value.")
            .to_string();
        let url_map_file_path = matches
            .value_of("url_map_file_path")
            .expect("Error getting `output file` value.")
            .to_string();
        let port = matches
            .value_of("port")
            .expect("Error getting `port` value.")
            .to_string();
        ConfigurationParameters {
            local_host,
            port,
            url_map_file_path,
        }
    }
}

fn get_eligible_arguments_for_app(app_name: &str) -> clap::ArgMatches {
    App::new(app_name)
        .about("Takes input parameters for URL shortner.")
        .arg(
            Arg::with_name("local_host")
                .long("local-host")
                .value_name("Local Host")
                .help("IP on which the program will run.")
                .default_value("127.0.0.1")
                .required(false),
        )
        .arg(
            Arg::with_name("url_map_file_path")
                .long("url-map-file-path")
                .value_name("Output File")
                .help("Path to the output file.")
                .required(true),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .value_name("Port")
                .help("port on which the program will run.")
                .default_value("8000")
                .required(false),
        )
        .get_matches()
}
