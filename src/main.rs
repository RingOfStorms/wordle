use rand::seq::SliceRandom;

mod dictionary;
mod utils;
mod wordle;

use crate::dictionary::generate_wordle_dictionary;
use crate::utils::{clear_screen, get_input, str_unique_by_characters};
use crate::wordle::{worlde_game_make_guess, WordleState};

fn main() {
    let wordle_dict = generate_wordle_dictionary();

    let unique_words: Vec<_> = wordle_dict
        .iter()
        .filter(|x| str_unique_by_characters(x))
        .map(|w| w)
        .collect();
    let worlde_answer = unique_words.choose(&mut rand::thread_rng()).unwrap();

    let mut wordle_game_state = WordleState::new(worlde_answer.as_str());

    clear_screen();
    loop {
        let mut input;

        loop {
            input = get_input("Guess a word:").to_uppercase();

            if input.len() == 5 && wordle_dict.contains(&input) {
                break;
            }

            if input.eq("!ANS") || input.eq("!ANSWER") {
                println!("Answer: {worlde_answer}");
            } else {
                println!("Invalid input {input}, try a different one...");
            }
        }

        worlde_game_make_guess(input.as_ref(), &mut wordle_game_state);
        clear_screen();
        print!("Board:\n{}", wordle_game_state);
        if wordle_game_state.game_over() {
            break;
        }
    }

    match wordle_game_state.won() {
        true => println!("YOU WON!"),
        false => println!("YOU LOST!"),
    }
}
