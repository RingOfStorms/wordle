use ansi_term::Color::{Green, Yellow, RGB};
use ansi_term::Colour;
use std::fmt;
use std::fmt::Formatter;

use crate::utils::str_to_five_char;

const GRAY: Colour = RGB(188, 188, 188);
const DARK_GRAY: Colour = RGB(117, 117, 117);

#[derive(Copy, PartialEq, Debug)]
pub enum CharState {
    EXCLUDES,
    CONTAINS,
    POSITIONED,
}

impl Clone for CharState {
    fn clone(&self) -> Self {
        match self {
            CharState::EXCLUDES => CharState::EXCLUDES,
            CharState::CONTAINS => CharState::CONTAINS,
            CharState::POSITIONED => CharState::POSITIONED,
        }
    }
}

pub struct WordleState {
    word: [char; 5],
    guesses: [Option<[char; 5]>; 5],
    guess_states: [Option<[CharState; 5]>; 5],
}

impl WordleState {
    pub fn new(word: &str) -> Self {
        WordleState {
            word: str_to_five_char(word),
            guesses: [None; 5],
            guess_states: [None; 5],
        }
    }

    pub fn get_turn(&self) -> Option<usize> {
        self.guesses.iter().position(|guess| guess.is_none())
    }

    pub fn game_over(&self) -> bool {
        self.get_turn().is_none() || self.won()
    }

    pub fn won(&self) -> bool {
        let last_index = self.get_turn().map(|t| t - 1).unwrap_or(4);
        !self.guess_states[last_index]
            .unwrap()
            .iter()
            .any(|s| !CharState::POSITIONED.eq(s))
    }

    pub fn set_guess(&mut self, index: usize, guess: [char; 5]) {
        self.guesses[index] = Option::from(guess);
        self.calculate_guess_states(false);
    }

    // pub fn calculate_missing_guess_states(&mut self) {
    //     self.calculate_guess_states(true)
    // }

    pub fn calculate_guess_states(&mut self, skip_existing: bool) {
        self.guesses.iter().enumerate().for_each(|(i, g)| {
            if g.is_some() && (!skip_existing || self.guess_states[i].is_none()) {
                // EXCLUDES is default so anything at the end is considered excluded
                let mut guess_state = [CharState::EXCLUDES; 5];
                let mut guess = g.unwrap().map(|c| Option::from(c));
                let mut word = self.word.map(|c| Option::from(c));

                // POSITIONED
                for char_index in 0..5 {
                    if guess[char_index].unwrap() == word[char_index].unwrap() {
                        guess[char_index] = None;
                        word[char_index] = None;
                        guess_state[char_index] = CharState::POSITIONED;
                    }
                }

                // CONTAINS
                for char_index in 0..5 {
                    if guess[char_index].is_some() {
                        for word_index in 0..5 {
                            if word[word_index].is_some()
                                && guess[char_index].unwrap() == word[word_index].unwrap()
                            {
                                word[word_index] = None;
                                guess[char_index] = None;
                                guess_state[char_index] = CharState::CONTAINS;
                                break;
                            }
                        }
                    }
                }

                self.guess_states[i] = Option::from(guess_state);
            }
        });
    }
}

impl fmt::Display for WordleState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.guesses.iter().enumerate().for_each(|(i, g)| {
            f.write_str(
                match g {
                    None => format!("{}\n", GRAY.paint("_ _ _ _ _")),
                    Some(guess) => match self.guess_states[i] {
                        None => format!(
                            "{}\n",
                            GRAY.paint(
                                guess
                                    .iter()
                                    .map(|c| format!("{} ", c.to_string()))
                                    .collect::<String>()
                            )
                        ),
                        Some(_) => format!(
                            "{}\n",
                            guess
                                .iter()
                                .enumerate()
                                .fold(String::new(), |mut str, (i_c, c)| {
                                    str.push_str(
                                        format!(
                                            "{} ",
                                            match self.guess_states[i].unwrap()[i_c] {
                                                CharState::EXCLUDES =>
                                                    DARK_GRAY.paint(c.to_string()),
                                                CharState::CONTAINS => Yellow.paint(c.to_string()),
                                                CharState::POSITIONED => Green.paint(c.to_string()),
                                            }
                                        )
                                        .as_str(),
                                    );
                                    str
                                })
                        ),
                    },
                }
                .as_str(),
            )
            .unwrap()
        });
        Ok(())
    }
}

pub fn worlde_game_make_guess(guess: &str, state: &mut WordleState) {
    let turn = state.guesses.iter().position(|guess| guess.is_none());
    match turn {
        Some(t) => println!("Current turn index: {}", t),
        None => println!("Game is over"),
    }

    if turn.is_some() {
        let turn = turn.unwrap();
        state.set_guess(turn, str_to_five_char(guess));
    }
}
