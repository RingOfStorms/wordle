use rand::seq::SliceRandom;
use regex::Regex;
use std::collections::HashSet;
use std::fs::{read_dir, File, OpenOptions};
use std::io::{stdin, BufRead, BufReader, Write};

mod utils;
mod wordle;

use utils::str_unique_by_characters;
use wordle::{worlde_game_make_guess, WordleState};

fn generate_wordle_dictionary() -> HashSet<String> {
    let existing_path = "./dictionaries/output/five_letter_words.txt";
    let existing = File::open(&existing_path);
    let paths: Vec<_> = read_dir("./dictionaries")
        .unwrap()
        .map(|p| p.unwrap())
        .filter(|p| p.file_type().unwrap().is_file())
        .collect();

    // Only regenerate five letter word dictionary if there are new or changed dictionaries
    let regenerate = existing.map_or(true, |e| {
        let modified = e.metadata().unwrap().modified().unwrap();
        paths
            .iter()
            .any(|dict| dict.metadata().unwrap().modified().unwrap().gt(&modified))
    });

    println!("Regenerating five letter word dictionary: {}", regenerate);
    let mut five_letter_words = HashSet::new();
    if regenerate {
        let _ = File::create(&existing_path); // create if not exist
        let mut f_writer = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&existing_path)
            .unwrap();

        let five_re = Regex::new(r"^(\w{5})(?:\s|$)").unwrap();
        paths.iter().for_each(|p| {
            println!("Loading five letter words from: {}", p.path().display());

            let f = File::open(p.path()).unwrap();
            let f = BufReader::new(f);
            f.lines().map(|l| l.unwrap()).for_each(|line| {
                match five_re.captures(&line) {
                    Some(x) => {
                        let word = x.get(1).unwrap().as_str().to_uppercase();
                        // println!("Capture: {} from {}", word, line);
                        five_letter_words.insert(word.clone());
                        f_writer.write(format!("{}\n", &word).as_ref()).unwrap();
                    }
                    None => (),
                }
            })
        });

        f_writer.flush().unwrap();
    } else {
        println!(
            "Loading five letter word dictionary from: {}",
            &existing_path
        );
        let dictionary_f = File::open(&existing_path).unwrap();
        let dictionary_f = BufReader::new(dictionary_f);
        dictionary_f.lines().for_each(|l| {
            five_letter_words.insert(l.unwrap());
        });
    }

    five_letter_words
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{prompt}");
    stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    input.trim().to_string()
}

fn main() {
    let wordle_dict = generate_wordle_dictionary();
    // println!("Wordle dict size: {}", wordle_dict.len());

    let unique_words: Vec<_> = wordle_dict
        .iter()
        .filter(|x| str_unique_by_characters(x))
        .map(|w| w)
        .collect();
    let worlde_answer = unique_words.choose(&mut rand::thread_rng()).unwrap();

    let mut wordle_game_state = WordleState::new(worlde_answer.as_str());

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

/*
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    let data = "Some data!";
    let f = File::create("/tmp/foo").expect("Unable to create file");
    let mut f = BufWriter::new(f);
    f.write_all(data.as_bytes()).expect("Unable to write data");
}
 */

/*
use lazy_static::lazy_static;

use regex::Regex;

fn extract_login(input: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)
            ^(?P<login>[^@\s]+)@
            ([[:word:]]+\.)*
            [[:word:]]+$
            ").unwrap();
    }
    RE.captures(input).and_then(|cap| {
        cap.name("login").map(|login| login.as_str())
    })
}

fn main() {
    assert_eq!(extract_login(r"I❤email@example.com"), Some(r"I❤email"));
    assert_eq!(
        extract_login(r"sdf+sdsfsd.as.sdsd@jhkk.d.rl"),
        Some(r"sdf+sdsfsd.as.sdsd")
    );
    assert_eq!(extract_login(r"More@Than@One@at.com"), None);
    assert_eq!(extract_login(r"Not an email@email"), None);
}
 */

/*
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("/etc/hosts").expect("Unable to open file");
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        println!("Line: {}", line);
    }
}
 */

/*
use std::fs;

fn main() {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
}

 */
