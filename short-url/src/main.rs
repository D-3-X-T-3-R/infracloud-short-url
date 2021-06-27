extern crate nanoid;

mod generate_url_code;

fn main() {
    println!("{:?}",generate_url_code::generate_url_code());
}
