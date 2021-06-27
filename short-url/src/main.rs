extern crate actix_web;
extern crate nanoid;

mod configuration_parameters;
mod generate_url_code;
mod routes;

use actix_web::{middleware, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_name = "short-url";

    let config_param = configuration_parameters::get_configuration_parameters(app_name);

    std::env::set_var("RUST_LOG", "actix_web=info");
    let host_url = config_param.local_host + ":" + &config_param.port;

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(routes::get_shorten_url)
        // add new services here
    })
    .bind(&host_url)
    .expect("Could not bind to host url, address already in use.")
    .run()
    .await
    // println!("{:?}",config_param);
}
