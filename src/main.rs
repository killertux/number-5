use std::env;
use warp::Filter;

mod phrases;
mod weather;
mod where_should_we_eat;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let get_place_to_eat =
        warp::path("where-should-we-eat").and_then(where_should_we_eat::get_place_to_eat);
    let get_place_to_eat_with_offset = warp::path!("where-should-we-eat" / u32)
        .and_then(where_should_we_eat::get_place_to_eat_with_offset);
    let get_place_to_eat_in_the_next_days =
        warp::path!("where-should-we-eat-in-the-next-days" / u32)
            .and_then(where_should_we_eat::get_place_to_eat_in_the_next_days);
    let is_it_raining = warp::path("is-it-raining").and_then(weather::is_it_raining);
    let phrases = warp::any().and_then(|| phrases::get_phrase());

    let routes = warp::get()
        .and(
            get_place_to_eat_with_offset
                .or(get_place_to_eat)
                .or(get_place_to_eat_in_the_next_days)
                .or(is_it_raining),
        )
        .or(phrases)
        .with(warp::log("number-5"));

    warp::serve(routes).run(([127, 0, 0, 1], get_port())).await;
}

fn get_port() -> u16 {
    match env::var_os("PORT") {
        Some(val) => val.to_string_lossy().parse().expect("Error parsing port"),
        None => 4000,
    }
}
