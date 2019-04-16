#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]
#[macro_use] extern crate rocket;

mod phrases;
mod where_should_we_eat;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![where_should_we_eat::get_place_to_eat])
        .register(catchers![phrases::get_phrase])
        .launch();
}
