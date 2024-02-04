use std::io;

use crate::game::{Game, game_load, game_save};
use crate::menu::{Action, clear, eval};

mod menu;
mod game;
mod api;

fn main() {
    let menu_result = menu::render_menu();
    match menu_result.0  {
        Action::START => new_game(),
        Action::RESUME => update(&mut game_load(eval("last")).unwrap_or(Game::new())),
        Action::SAVED => update(&mut game_load(menu_result.1.to_string()).unwrap_or(Game::new()))
    }
    println!("Click enter to go back to the menu");
    get_char();
    main()
}

fn new_game() {
    clear();
    println!("The executioner is thinking......");
    println!("He got one!");
    println!("Loading...");
    clear();
    update(&mut Game::new())
}

fn update(game: &mut Game) {
    clear();

    if game.guesses < 1 {
        println!("You've lost, the word was {}", game.word);
        return;
    }

    if game.is_won() {
        println!("You won");
        game.render_word();
        return;
    }

    println!("Just press enter if you want to save and go back to the menu");
    println!("You have {} wrong guesses left", &game.guesses);
    println!("You've guessed wrong the following letters: {:?}", &game.wrong_guesses);

    game.render_word();

    let input = get_char();

    if input == '\n' {
        game_save(game);
        return;
    }

    match game.word.contains(input) {
        false => {
            if game.wrong_guesses.contains(&input) {
                update(game)
            }
            game.wrong_guesses.push(input);
            game.guesses = game.guesses - 1;
            update(game)
        }
        true => {
            if !game.guessed.contains(&input) {
                game.guessed.push(input);
            }
            update(game)
        }
    }
}


fn get_char() -> char {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.chars().next().unwrap_or_else(|| get_char())
}