use super::generate_url_code::generate_shorten_url;
use actix_web::{get, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct URL {
    url: String
}

// Get shorten URL
#[get("/get_short_url/")]
async fn get_shorten_url(body_param: web::Json<URL>) -> Result<HttpResponse, Error> {
    let data = generate_shorten_url(body_param.url.to_string()).unwrap();
    Ok(HttpResponse::Ok().json(data))
}
