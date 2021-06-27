extern crate nanoid;

mod generate_url_code;
mod configuration_parameters;

fn main() {
    let app_name = "short-url";

    let config_param = configuration_parameters::get_configuration_parameters(app_name);

    println!("{:?}",config_param);
}
