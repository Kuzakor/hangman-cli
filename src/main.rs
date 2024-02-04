mod menu;

use std::io;
use crate::menu::{eval, clear};
use serde::Deserialize;

fn main() {
    if menu::render_menu() != menu::Action::START {
        return
    }
    game();
    println!("Click enter to go back to the menu");
    get_char();
}

fn game() {
    clear();
    println!("The executioner is thinking......");
    let word = get_word();
    println!("He got one!");
    println!("Loading...");
    let guesses = eval("guess").parse().unwrap();
    let mut wrong_guesses = Vec::new();
    let mut guessed = Vec::new();
    clear();
    update(&word, guesses, &mut wrong_guesses, &mut guessed);
}

fn update(word: &String, guesses: i32, wrong_guesses: &mut Vec<char>, guessed: &mut Vec<char>) {
    clear();

    if guesses < 1 {
        println!("You've lost, the word was {}", word);
        return;
    }

    if is_won(word, guessed) {
        println!("You won");
        render_word(word, guessed);
        return;
    }

    println!("You have {} wrong guesses left", guesses);
    println!("You've guessed wrong the following letters: {:?}", wrong_guesses);

    render_word(word, guessed);

    let input = get_char();
    match word.contains(input) {
        false => {
            if wrong_guesses.contains(&input) {
                update(word, guesses, wrong_guesses, guessed)
            }
            wrong_guesses.push(input);
            update(word, guesses - 1, wrong_guesses, guessed)
        }
        true => {
            if !guessed.contains(&input) {
                guessed.push(input);
            }
            update(word, guesses, wrong_guesses, guessed)
        }
    }
}

fn is_won(word: &String, guessed: &mut Vec<char>) -> bool{
    for i in word.chars() {
        if !guessed.contains(&i) {
            return false;
        }
    }
    true
}


fn render_word(word: &String, guessed: &mut Vec<char>) {
    for i in word.chars() {
        match guessed.contains(&i) {
            true => print!("{} ", i),
            false => print!("_ ")
        }
    }
    println!();
}

fn get_word() -> String{
    setup_mode();

    let url = match eval("easy").as_str(){
        "0" => generate_hard_url(),
        "1" => generate_easy_url(),
        _ => panic!("How the fuck")
    };

    match reqwest::blocking::get(url).unwrap().text() {
        Ok(value) => word_cleanup(value),
        Err(_) => get_word()
    }

}

fn word_cleanup(s: String) -> String{
    let mut s = s.clone();
    for _ in 0..2 {
        s.remove(0);
        s.pop();
    }
    s
}


fn setup_mode() {
    let db_connection = sled::open("data").unwrap();
    if db_connection.get("easy").unwrap().is_none() {
        let _ = db_connection.insert("easy", "0");
    }
    drop(db_connection);
}


fn generate_easy_url() -> String {
    match eval("length").as_str() {
        "0" => String::from("https://random-word-api.vercel.app/api?words=1"),
        _ => format!("https://random-word-api.vercel.app/api?words=1&length={}",  eval("length").as_str()),
    }
}

fn generate_hard_url() -> String {
    match eval("length").as_str() {
        "0" => format!("https://random-word-api.herokuapp.com/word?lang={}", eval("lang").as_str()),
        _ => format!("https://random-word-api.herokuapp.com/word?lang={}&length={}", eval("lang").as_str(), eval("length").as_str()),

    }
}

fn get_char() -> char{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.chars().next().unwrap_or_else(|| get_char())
}