use std::error::Error;
use std::io;
use sled::*;

#[derive(Debug, PartialEq)]
pub enum Action {
    START,
    RESUME,
    SAVED,
}


pub fn render_menu() -> Action {
    clear();
    println!("-----------------------------------");
    println!("| Select option:                  |");
    println!("| 1. Start new game               |");
    println!("| 2. Resume last game (under dev) |");
    println!("| 3. Saved games (under dev)      |");
    println!("| 4. Settings                     |");
    println!("| 5. Exit                         |");
    println!("-----------------------------------");
    match get_int(){
        1 => Action::START,
        2 => Action::RESUME,
        3 => Action::SAVED,
        4 => render_settings(),
        5 => std::process::exit(0),
        _ => render_menu()

    }
}

pub fn render_settings() -> Action {

    clear();
    println!("---------------------------------------------");
    println!("| Select option:                              ");
    println!("| 1. Language ({})                       ", eval("lang"));
    println!("| 2. How many guessses ({})              ", eval("guess"));
    println!("| 3. How long the world (custom only for English)({})                    ", eval("length"));
    println!("| 4. Turn on/off easy mode (English only) ({}) ", eval("easy"));

    println!("| 5. Back                                     ");
    println!("---------------------------------------------" );
    match get_int(){
        1 => render_lang(),
        2 => {
            clear();
            println!("Insert a number (deafult: 13): ");
            let _ = sled::open("data").unwrap().insert("guess", &*get_int().to_string());
            render_settings()
        }
        3 =>  {
            clear();
            println!("Insert a number (deafult: 0 (random): ");
            let _ = sled::open("data").unwrap().insert("length", &*get_int().to_string());
            render_settings()
        }
        4 => {
            match eval("easy") == "0" {
                true => sled::open("data").unwrap().insert("easy", "1"),
                false => sled::open("data").unwrap().insert("easy", "0")
            }.expect("Something went wrong with database");

            render_settings()
        }
        _ => render_menu()
    }

}

pub fn render_lang() -> Action {
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
    println!("-----------------" );
    let input = get_int();
    if input == 7 {
        return render_menu()
    }
    if input > 7 {
        return render_lang()
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

fn get_int() -> i32{
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("read error, somehow?");
    num.trim().parse().unwrap_or(13)
}
pub fn eval(key: &str) -> String{
    let con = sled::open("data").unwrap().get(key).unwrap();
    match con {
        Some(x) =>   String::from(std::str::from_utf8 (&x).unwrap()),
        None => {
            match key {
                "lang" => String::from("en"),
                "length" => String::from("0"),
                "guess" => String::from("13"),
                _ => String::from("Not set")
            }
        }
    }

}
