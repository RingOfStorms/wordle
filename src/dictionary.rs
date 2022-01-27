use regex::Regex;
use std::collections::HashSet;
use std::fs::{read_dir, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

pub fn generate_wordle_dictionary() -> HashSet<String> {
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

    println!("five_letter_words size: {}", five_letter_words.len());

    five_letter_words
}
