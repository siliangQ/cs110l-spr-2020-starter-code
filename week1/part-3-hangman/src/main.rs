// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fmt::Display;
use std::io;
use std::io::Write;
use std::{fmt::Formatter, fs};

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

struct Guess {
    pub guessed_letters: Vec<String>,
    pub result: Vec<char>,
}

impl Guess {
    fn new(len: usize) -> Self {
        let v = vec!['-'; len];
        Guess {
            guessed_letters: Vec::new(),
            result: v,
        }
    }
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {:?}", secret_word_chars);

    // Your code here! :)
    println!("Welcome to CS110L Hangman");
    let mut guess = Guess::new(secret_word_chars.len());
    let mut total_guesses = 0;
    loop {
        if total_guesses >= NUM_INCORRECT_GUESSES {
            println!("Sorry, you ran out of guesses!");
            break;
        }

        if !guess.result.contains(&'-') {
            println!(
                "Congratulations you guesses the secret word: {:?}",
                secret_word_chars
            );
            break;
        }
        println!(
            "The word so far is {}",
            guess
                .result
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join("")
        );
        println!(
            "You have guesses the following letters: {}",
            guess.guessed_letters.join("")
        );
        println!(
            "You have {} guesess left",
            NUM_INCORRECT_GUESSES - total_guesses
        );
        print!("Please guess a letter: ");
        io::stdout().flush().expect("Error flushing stdout");
        let mut new_guess = String::new();
        io::stdin()
            .read_line(&mut new_guess)
            .expect("Error reading input");
        new_guess.pop();
        let mut index_vec = (0..secret_word_chars.len()).collect::<Vec<usize>>();
        index_vec.retain(|&i| secret_word_chars[i].to_string() == new_guess.clone());
        if index_vec.len() == 0 {
            total_guesses += 1;
            println!("Sorry, that letter is not in the word");
        } else {
            guess.guessed_letters.push(new_guess);
            for i in index_vec {
                guess.result[i] = secret_word_chars[i];
            }
        }
    }
}
