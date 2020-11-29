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
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);
    let mut charCount = secret_word_chars.len();
    let mut incorrect_guess = 0;
    let mut word_chars: Vec<char> = Vec::new();
    let mut already: Vec<char> = Vec::new();
    for i in 0..charCount {
        word_chars.push('_');
    }
    // Your code here! :)
    while charCount != 0 && incorrect_guess != NUM_INCORRECT_GUESSES {
        print!("The word so far is ");
        for i in 0..word_chars.len() {
            print!("{}", word_chars[i]);
        }
        println!();
        print!("You have guessed the following letters: ");
        for i in 0..already.len() {
            print!("{}", already[i]);
        }
        println!();
        println!("You have {:?} guesses left", NUM_INCORRECT_GUESSES - incorrect_guess);
        print!("Please guess a letter: ");
        io::stdout()
            .flush()
            .expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        println!();
        let mut is_right = false;
        for i in 0..secret_word_chars.len() {
            if guess.chars().nth(0).unwrap() == secret_word_chars[i] && word_chars[i] == '_' {
                word_chars[i] = guess.chars().nth(0).unwrap();
                is_right = true;
                break;
            }
        }
        already.push(guess.chars().nth(0).unwrap());
        if is_right == false {
            println!("Sorry, that letter is not in the word");
            incorrect_guess = incorrect_guess + 1;
        } else {
            charCount = charCount - 1;
        }
        println!();
    }
    if incorrect_guess == NUM_INCORRECT_GUESSES {
        println!("Sorry, you ran out of guesses!");
    } else {
        println!("Congratulations you guessed the secret word: {}!", secret_word);
    }


}
