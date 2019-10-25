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

#[derive(Serialize)]
pub struct PlaceToEatDay {
    days_from_today: u32,
    place_to_eat: PlaceToEat,
}

#[get("/where-should-we-eat")]
pub fn get_place_to_eat() -> Json<PlaceToEat> {
    let offset = 0;
    Json(choose_place_to_eat(offset))
}

#[get("/where-should-we-eat/<offset>")]
pub fn get_place_to_eat_with_offset(offset: u32) -> Json<PlaceToEat> {
    Json(choose_place_to_eat(offset))
}

#[get("/where-should-we-eat-in-the-next-days/<days>")]
pub fn get_place_to_eat_in_the_next_days(days: u32) -> Json<Vec<PlaceToEatDay>> {
    let next_days: Vec<PlaceToEatDay> = (0..days).into_iter()
                                            .map(|day| PlaceToEatDay{
                                                days_from_today: day,
                                                place_to_eat: choose_place_to_eat(day),
                                            })
                                            .collect();

    Json(next_days)
}

fn choose_place_to_eat(offset: u32) -> PlaceToEat {
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
    let chosen_list = places[distribution.sample(&mut get_rnd(&offset, 1).0)].0.clone();

    PlaceToEat{
        place_to_eat: get_from_random_list(chosen_list, &offset)
    }
}

fn get_from_random_list(mut places: Vec<&str>, offset: &u32) -> String {
    let (mut rnd, group_index) = get_rnd(& offset, places.len() as u32);
    places.shuffle(&mut rnd);
    places[group_index as usize].to_string()
}

fn get_rnd(offset: &u32, group_size: u32) -> (StdRng, u32) {
    let super_seed = 42;
    let mut rnd = StdRng::seed_from_u64(super_seed);
    let days_since_birth = get_days_since_my_birth() + offset;
    let group_index = days_since_birth % group_size;
    let start_of_group = days_since_birth - group_index;

    for _i in 0..start_of_group {
        rnd.next_u32();
    }
    (rnd, group_index)
}

fn get_days_since_my_birth() -> u32 {
    let birthday = Utc.ymd(1986, 5, 9);
    let now =  Utc::now().date();
    now.signed_duration_since(birthday)
        .num_days() as u32
}
