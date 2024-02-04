use crate::api::get_word;
use crate::menu::eval;
use bitcode::{Encode, Decode};

#[derive(bitcode::Encode, bitcode::Decode, Debug)]
pub struct Game {
    pub id: i32,
    pub word: String,
    pub guesses: i32,
    pub wrong_guesses: Vec<char>,
    pub guessed: Vec<char>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            id: eval("last").parse::<i32>().unwrap() + 1,
            word: get_word(),
            guesses: eval("guess").parse().unwrap(),
            wrong_guesses: Vec::new(),
            guessed: Vec::new(),
        }
    }

    pub fn is_won(&self) -> bool {
        for i in self.word.chars() {
            if !self.guessed.contains(&i) {
                return false;
            }
        }
        true
    }


    pub fn render_word(&self) {
        for i in self.word.chars() {
            match self.guessed.contains(&i) {
                true => print!("{} ", i),
                false => print!("_ ")
            }
        }
        println!();
    }
}

pub fn game_save(game: &Game) {
    let db = sled::open("data").unwrap();
    let _ = db.insert("last", game.id.to_string().as_str());
    let serialized_game = bitcode::encode(&game).unwrap();
    let _ = db.insert(game.id.to_string(), serialized_game);
}

pub fn game_remove(id: String) {
    let db = sled::open("data").unwrap();
    let _ = db.remove(id.to_string());
    drop(db);
    let games_to_switch = load_all_games(id.parse().unwrap());
    let db = sled::open("data").unwrap();
    for i in &games_to_switch {
        let _ = db.remove(i.id.to_string());
    }
    drop(db);
    for mut i in games_to_switch {
        i.id = i.id - 1;
        game_save(&i);
    }
}


pub fn game_load(id: String) -> Option<Game> {
    let db = sled::open("data").unwrap();
    let serialized_game = db.get(id);
    drop(db);
    if serialized_game.clone().is_err() {
        return None
    }
    if serialized_game.clone().unwrap().is_none() {
        return None
    }
    Some(bitcode::decode(&serialized_game.unwrap().unwrap()).unwrap())

}

pub fn load_all_games(starting_point: i32) -> Vec<Game> {
    let last_id = eval("last");
    let mut games:Vec<Game> = Vec::new();
    for i in starting_point-1..last_id.parse().unwrap() {
        match game_load((i+1).to_string()) {
            Some(x) => games.push(x),
            None =>  continue
        }
    }
    games

}
