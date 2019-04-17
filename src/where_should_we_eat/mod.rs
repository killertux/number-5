use rocket_contrib::json::Json;
use serde::Serialize;
use rand::prelude::*;
use rand::rngs::StdRng;
use rand::distributions::WeightedIndex;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
pub struct PlaceToEat {
    place_to_eat: String,
}

#[get("/where-should-we-eat")]
pub fn get_place_to_eat() -> Json<PlaceToEat> {
    let premium_places = vec![
        "Madero",
        "Marbo",
        "Mustang Sally",
        "Madalosso",
        "Recanto Gaúcho",
        "Cabana Motenfusco",
        "Bife-sujo",
    ];
    let normal_places = vec![
        "Jokers",
        "Veneza",
        "Everest",
        "São Francisco",
        "Nona",
        "Origami",
        "Deodoro Grill",
        "Pomba Gira",
        "Ponto Setti",
        "Villa Urbana",
        "Shawarma",
        "Japa da Costela",
        "No Canto",
        "Em casa",
        "Árabe",
        "Quintal do Monge",
        "Don Juan",
    ];
    let places = [(premium_places, 1), (normal_places, 19)];
    let distribution = WeightedIndex::new(places.iter().map(|place| place.1)).unwrap();
    Json(PlaceToEat{place_to_eat: get_from_random_list(&places[distribution.sample(&mut get_rnd())].0)})
}

fn get_from_random_list(places: &[&str]) -> String {
    match places.choose(&mut &mut get_rnd()) {
        Some(x) => x.to_string(),
        None => "Ops!".to_string(),
    }
}

fn get_rnd() -> StdRng {
    let seconds_in_a_day = 60 * 60 * 24;

    let start = SystemTime::now();
    let s = start.duration_since(UNIX_EPOCH).expect("Something very wrong happend!");

    StdRng::seed_from_u64((s.as_secs() / seconds_in_a_day) * seconds_in_a_day)
}
