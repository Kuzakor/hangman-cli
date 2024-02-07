use crate::api::get_word;
use crate::menu::eval;
use bitcode::{Encode, Decode};


/* Game object
- Stores data neccessary to run the update() func
- Derives Encpde and Decode for easy database reading and saving
*/
#[derive(bitcode::Encode, bitcode::Decode)]
pub struct Game {
    pub id: i32,
    pub word: String,
    pub guesses: i32,
    pub wrong_guesses: Vec<char>,
    pub guessed: Vec<char>,
}

/* Game implementation*/
impl Game {

    /*New
    - new game object
    - random
    - unsolved
    */
    pub fn new() -> Self {
        Self {
            id: eval("last").parse::<i32>().unwrap() + 1,
            word: get_word(),
            guesses: eval("guess").parse().unwrap(),
            wrong_guesses: Vec::new(),
            guessed: Vec::new(),
        }
    }


    /* Is the game won
    - iterates over the word
    - if a char from word is not in the solved pool it means it is not solved
    - if all chars are (so it reaches the end of the loop) it is
    */
    pub fn is_won(&self) -> bool {
        for i in self.word.chars() {
            if !self.guessed.contains(&i) {
                return false;
            }
        }
        true
    }

    /*Blank / word rendering
    - Iterates over the word and if the letter exist in solved pool it places it
    - Otherwise it places an _
    - All letters and _ are divided by spaces
    - Finishes with a new line
    */
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

/*Database game saving
- Connects to the db
- Sets the last id key to the id of saved game
- Encrypts the game
- Saves the game to the database
* let _ => does nothing but removes the warning about unused result
*/
pub fn game_save(game: &Game) {
    let db = sled::open("data").unwrap();
    let _ = db.insert("last", game.id.to_string().as_str());
    let serialized_game = bitcode::encode(&game).unwrap();
    let _ = db.insert(game.id.to_string(), serialized_game);
}


/* Game removal
- Connects to the db
- Removes the given game
- Loads all the games after it
- Removes them from db
- Changes their id to 1 lower
- Saves them to the db
* drop(db) => disconnect from db so it can be used by other functions
*/
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

/* Game loading
- Connects to db
- Gets the encrypted version of the game
- Checks if it was successful add if its not empty
- Decrypts
*/
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


/* Multiple game loading
- Checks how many games are there (last_id)
- Creates an empty vec for the games to be stored in
- Starts loading games from given point up untill the end.
- If there is a no game in between some - skips it
- Returns the games
*/
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
