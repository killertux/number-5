use rocket_contrib::json::Json;
use serde::Serialize;
use rand::prelude::*;
use rand::rngs::StdRng;
use rand::distributions::WeightedIndex;
use chrono::prelude::*;

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

    let chosen_list = &places[distribution.sample(&mut get_rnd())].0;

    Json(PlaceToEat{
        place_to_eat: get_from_random_list(chosen_list)
    })
}

fn get_from_random_list(places: &[&str]) -> String {
    places.choose(&mut get_rnd())
            .unwrap_or(&"Rand error!")
            .to_string()
}

fn get_rnd() -> StdRng {
    let super_seed = 42;
    let mut rnd = StdRng::seed_from_u64(super_seed);
    for _i in 0..get_days_since_my_birth() {
        rnd.next_u32();
    }
    rnd
}

fn get_days_since_my_birth() -> u32 {
    let birthday = Utc.ymd(1986, 5, 9);
    let now =  Utc::now().date();
    now.signed_duration_since(birthday)
        .num_days() as u32
}
