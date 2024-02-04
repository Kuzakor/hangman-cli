use std::io;
use crate::game::{load_all_games, game_remove};
#[derive(Debug, PartialEq)]
pub enum Action {
    START,
    RESUME,
    SAVED,
}
pub fn render_menu() -> (Action, i32) {
    clear();
    println!("-----------------------------------");
    println!("| Select option:                  |");
    println!("| 1. Start new game               |");
    println!("| 2. Resume last game             |");
    println!("| 3. Saved games                  |");
    println!("| 4. Settings                     |");
    println!("| 5. Exit                         |");
    println!("-----------------------------------");
    match get_int() {
        1 => (Action::START, 0),
        2 => (Action::RESUME, 0),
        3 => render_saved(),
        4 => render_settings(),
        5 => std::process::exit(0),
        _ => render_menu()
    }
}

pub fn render_saved() -> (Action, i32){
    clear();
    println!("---------------------------------------------------------");
    for game in load_all_games(1) {
        println!("| {}. | remaining guesses: {}. guessed letters: {} ot of {}", game.id, game.guesses, game.guessed.len(), game.word.len())
    }
    println!("---------------------------------------------------------");
    println!("Select game (number not from the list - exit): ");
    let id = get_int();
    if id > eval("last").parse().unwrap() {
        return render_menu();
    }
    clear();
    println!("-----------------------------------");
    println!("| Game number {} has been chosen |", id);
    println!("-----------------------------------");
    println!("| Select action:                  |");
    println!("| 1. Play                         |");
    println!("| 2. Remove                       |");
    println!("| 3. Select different             |");
    println!("-----------------------------------");
    let action = get_int();
    match action {
        1 => (Action::SAVED, id),
        2 => {
            game_remove(id.to_string());
            render_saved()
        }
        3 => render_saved(),
        _ => render_menu()
    }

}


pub fn render_settings() -> (Action, i32) {
    clear();
    println!("---------------------------------------------");
    println!("| Select option:");
    println!("| 1. Language ({})", eval("lang"));
    println!("| 2. How many guessses ({})", eval("guess"));
    println!("| 3. How long the world (custom only for English)({})", eval("length"));
    println!("| 4. Turn on/off easy mode (English only) ({}) ", eval("easy"));
    println!("| 5. Back");
    println!("---------------------------------------------");
    match get_int() {
        1 => render_lang(),
        2 => set_guesses(),
        3 => set_length(),
        4 => set_mode(),
        _ => render_menu()
    }
}

fn set_guesses() -> (Action, i32) {
    clear();
    println!("Insert a number (deafult: 13): ");
    let _ = sled::open("data").unwrap().insert("guess", &*get_int().to_string());
    render_settings()
}

fn set_length() -> (Action, i32) {
    clear();
    println!("Insert a number (deafult: 0 (random): ");
    let _ = sled::open("data").unwrap().insert("length", &*get_int().to_string());
    render_settings()
}

fn set_mode() -> (Action, i32) {
    match eval("easy") == "0" {
        true => sled::open("data").unwrap().insert("easy", "1"),
        false => sled::open("data").unwrap().insert("easy", "0")
    }.expect("Something went wrong with database");
    render_settings()
}

pub fn render_lang() -> (Action, i32) {
    clear();
    println!("------------------");
    println!("| Select option: |");
    println!("| 1. English     |");
    println!("| 2. French      |");
    println!("| 3. Chinese     |");
    println!("| 4. Italian     |");
    println!("| 5. Spanish     |");
    println!("| 6. German      |");
    println!("| 7. Back        |");
    println!("-----------------");
    let input = get_int();
    if input == 7 {
        return render_menu();
    }
    if input > 7 {
        return render_lang();
    }
    let value = match input {
        1 => "en",
        2 => "fr",
        3 => "zh",
        4 => "it",
        5 => "es",
        6 => "de",
        _ => "en"
    };
    let _ = sled::open("data").unwrap().insert("lang", value);
    render_settings()
}

pub fn clear() {
    print!("{}[2J", 27 as char);
}

fn get_int() -> i32 {
    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();
    num.trim().parse().unwrap_or(13)
}

pub fn eval(key: &str) -> String {
    match sled::open("data").unwrap().get(key).unwrap() {
        Some(x) => String::from(std::str::from_utf8(&x).unwrap()),
        None => set_default(key)
    }
}

fn set_default(key: &str) -> String {
    match key {
        "lang" => String::from("en"),
        "length" => String::from("0"),
        "guess" => String::from("13"),
        "last" => String::from("0"),
        _ => String::from("Not set")
    }
}
