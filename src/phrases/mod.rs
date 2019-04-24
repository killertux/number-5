use rocket_contrib::json::Json;
use serde::Serialize;
use rand::seq::SliceRandom;

#[derive(Serialize)]
pub struct Phrase {
    message: String,
}

#[catch(404)]
pub fn get_phrase() -> Json<Phrase> {
    Json(Phrase {
        message: get_random_phrase(),
    })
}

fn get_random_phrase() -> String {
    let phrases = vec![
        "Hey Laserlips. Your mama was a snowblower.",
        "No shit. Where see shit?",
        "Number 5 is alive.",
        "Many fragments. Some large, some small.",
        "Fish. Salmon. Sushie.",
        "Colt .45. Semi-automatic.",
        "Play-doh",
        "Malfunction. Need input.",
        "Nice software.",
        "Beautiful animal... canine... dog... mutt.",
        "Stupid - foolish, gullible, doltish, dumbell.",
        "Well, if you gotta go, don't squeeze the Charmin.",
        "Number 5 stupid name... want to be Kevin or Dave!",
        "Bird. Raven. Nevermore.",
        "Stephanie... change color!",
        "Attractive! Nice software. Mmm.",
        "Hmmmm. Oh, I get it! Ho ho ho ho ho ho ho ho ho ho ho! Hee hee hee hee hee hee hee hee hee! Nyuk, nyuk nyuk nyuk nyuk nyuk nyuk nyuk nyuk!",
        "Whatever God wants, he keeps!",
        "Program say to kill, to disassemble, to make dead. Number 5 cannot.",
        "Is *wrong*! Newton Crosby, Ph.D not know this?",
        "*I* told me.",
        "Wouldn't you like to be a Pepper too?",
        "Disassemble?",
        "Shut up - silence, hush, sit on it, can it...",
        "Frankie, you broke the unwritten law. You ratted on your friends. When you do that Frankie, your enemies don't respect you. You got no friends no more. You got nobody, Frankie.",
        "Escaped Robot Fights for His Life. Film at Eleven.",
        "Jerk of the world: Turkey, idiot, pain in the ass.",
        "Ah don't worry little lady, I'll fix their wagon.",
        "Well, I guess that waps you up, you wascally wobot - huhgh-huhgh-huhgh-huhgh-huhgh-huhgh-huhgh-huhgh-huhgh!",
        "Benjamin Jabituya, delete.",
        "Exit. Depart. Leave.",
        "Newton Crosby, no run, no talk, drive.",
        "Today, Crosby! Today!",
        "Hmmm... Wood pulp, plant - vegetable - tomato, water, salt, monosodium glutamate...",
        "...and resemble - look like - butterfly, bird, maple leaf!",
        "NOVA? NO! No disassemble!",
        "Come on, treads, don't fail me now!",
        "Hi!",
        "Piece of cake!",
        "Maybe Johnny... Yeah, Johnny 5.",
        "Stephanie?",
        "Kick ass? Donkey, mule, burro.",
        "Not malfunction Stephanie. Number 5 is alive.",
        "No disassemble Number Five!",
        "Oooooo! Beautiful. Light bulb.",
        "Beautiful No-sun.",
        "Error. Grasshopper disassembled... Re-assemble!",
        "Okay, to make these golden fluffy pancakes... add flour, milk and eggs... Mix thoroughly...",
        "Ooooo... Still lumpy!",
        "No, no, please. No autographs, sir! Come Stephanie! We be jammin'",
        "Drinking glass... Pasta, spaghetti!",
        "Liquid, spaghetti sauce...",
        "Oooo... Baskets... fruit!",
        "Oranges, Apples, Lemons, Limes...",
        "Escape, escape! Please, hide! Refuge!",
        "N.O.V.A. robotics, disassemble, dead! Disassemble, Number 5 dead!",
        "No.",
        "Yes!",
        "Yes...",
        "Yes... not.",
        "Or-e-gon.",
        "Beavers! Cherries! Input!",
        "More input! More input!",
        "Dead?",
        "Reassemble, Stephanie. Reassemble!",
        "Squash. Dead. Disassemble. Dead. Disassemble? Dead!",
        "Come on, Steph! Lighten up! Let's give it a whirl!",
        "Yeah!",
        "It's early!",
        "Dancing fool!",
        "This, home. Stephanie, home. Number 5, home.",
        "Number 5, know.",
        "More than a woman - More than a woman to me.",
        "Strategy.",
        "Need some. Quick!",
        "Kidnap: Shanghai, hijack.",
        "Communicate. Need input. I have questions, queries, *posers*.",
        "Well, above average.",
    ];

    phrases.choose(&mut rand::thread_rng())
        .unwrap_or(&"Ops!")
        .to_string()
}
