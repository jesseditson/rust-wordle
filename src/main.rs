use colored::Colorize;
use rand::Rng;
use std::fs;
use std::io;

const ALPHA: &str = "abcdefghijklmnopqrstuvwxyz";

fn get_word(all_words: &Vec<String>) -> String {
    let random = rand::thread_rng().gen_range(0, all_words.len());
    all_words[random].clone()
    // all_words.nth(random).unwrap().to_string()
}

fn read_words() -> Vec<String> {
    fs::read_to_string("./words.txt")
        .expect("Failed reading words file.")
        .lines()
        .map(|s| s.to_owned())
        .collect()
}

fn main() {
    let words = read_words();
    let word = get_word(&words);
    println!("Guess the word.");
    let mut guessed_letters = String::new();
    let mut correct_letters = String::new();
    let mut matched_letters = String::new();
    let mut remaining_guesses = 5;
    while remaining_guesses > 0 {
        println!(
            "Type a 5 letter word. {} guesses remain.",
            remaining_guesses
        );
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        guess = String::from(guess.trim().to_ascii_lowercase());
        if guess == "?" {
            for letter in ALPHA.chars() {
                if matched_letters.contains(letter) {
                    print!("{}", format!("{}", letter).blue());
                } else if correct_letters.contains(letter) {
                    print!("{}", format!("{}", letter).red());
                } else if guessed_letters.contains(letter) {
                    print!("{}", format!("{}", letter).bright_black());
                } else {
                    print!("{}", format!("{}", letter).white());
                }
            }
            print!("\n");
        }
        if guess.len() == 5 {
            if guess == word {
                println!("Correct! The word is {}", format!("{}", word).blue());
                return;
            }
            if !words.contains(&String::from(&guess)) {
                println!("that is not a valid word.");
                continue;
            }
            let mut correct_guess_chars = String::new();
            for (i, char) in word.chars().enumerate() {
                let guess_char: char = guess.chars().nth(i).unwrap();
                if guess_char == char {
                    correct_guess_chars.push(guess_char);
                    matched_letters.push(guess_char);
                } else if word.chars().any(|c| c == guess_char) {
                    correct_letters.push(guess_char);
                } else {
                    guessed_letters.push(guess_char);
                }
            }
            for (i, char) in word.chars().enumerate() {
                let guess_char: char = guess.chars().nth(i).unwrap();
                let s = guess_char.to_string();
                if guess_char == char {
                    print!("{}", format!("{}", s).blue());
                } else if !correct_guess_chars.contains(guess_char)
                    && word.chars().any(|c| c == guess_char)
                {
                    print!("{}", format!("{}", s).red());
                } else {
                    print!("_");
                }
            }
            print!("\n");
            remaining_guesses = remaining_guesses - 1;
        }
    }
    println!(
        "You ran out of guesses. The word was \"{}\"",
        format!("{}", word).red()
    );
}
