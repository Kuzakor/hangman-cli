mod menu;
use crate::menu::{eval, clear};
use serde::Deserialize;
use reqwest::blocking::get;
fn main() {
    if menu::render_menu() != menu::Action::START {
        return
    }
    game()
}

fn game() {
    clear();
    println!("The executioner is thinking......");
    let word = get_word();
    println!("He got one!");
    println!("Loading...");
    let guesses = eval("guess");
    clear();
    update();
}

fn update(word: String, guesses:i32) {
    clear();
    for i in 0..word.len() {
        print!("_ ")
    }
}

fn get_word() -> String{
    let db_connection = sled::open("data").unwrap();
    if db_connection.get("easy").unwrap().is_none() {
        let _ = db_connection.insert("easy", "0");

    }
    drop(db_connection);

    let url = match eval("easy").as_str(){
        "0" => {
            match eval("length").as_str() {
                "0" => format!("https://random-word-api.herokuapp.com/word?lang={}", eval("lang").as_str()),
                _ => format!("https://random-word-api.herokuapp.com/word?lang={}&length={}", eval("lang").as_str(), eval("length").as_str()),

            }
        }
        "1" => {
            match  eval("length").as_str() {
                "0" => String::from("https://random-word-api.vercel.app/api?words=1"),
                _ => format!("https://random-word-api.vercel.app/api?words=1&length={}",  eval("length").as_str()),
            }
        }
        _ => panic!("How the fuck")
    };

    match reqwest::blocking::get(url).unwrap().text() {
        Ok(mut value) => {
            for i in 0..2 {
                value.remove(0);
                value.pop();
            }
            return value;
        }
        Err(err) => println!("Error when connecting to the API")
    }

    std::process::exit(0)

}

