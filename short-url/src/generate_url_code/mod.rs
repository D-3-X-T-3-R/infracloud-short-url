use nanoid::nanoid;

pub fn generate_url_code() -> Result<String, String> {
    let url_code = nanoid!(8);
    Ok(url_code)
}