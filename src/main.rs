#[macro_use]
extern crate lazy_static;

mod configuration;
use configuration::{Config};

fn main() {
    let config = Config::get_config();
    println!("{:?}", config);
}
