#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]
#[macro_use] extern crate rocket;
use rocket::Config;
use std::env;

mod phrases;
mod where_should_we_eat;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

fn configure() -> Config {
    let mut config = Config::active().expect("could not load configuration");
    if let Ok(port_str) = env::var("PORT") {
        let port = port_str.parse().expect("could not parse PORT");
        config.set_port(port);
    }
    config
}

fn main() {
    rocket::custom(configure())
        .mount("/", routes![where_should_we_eat::get_place_to_eat])
        .register(catchers![phrases::get_phrase])
        .launch();
}
