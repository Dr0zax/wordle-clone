use std::io::{self, Write};
use rand::seq::{IndexedRandom};
use rand::rng;

const WORD_LENGTH: usize = 5;
const MAX_GUESSES: usize = 6;

const WORDS: &[&str] = &[
    "apple", "peach", "world", "temple", "nephi"
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LetterState {
    Correct, //green
    Present, //yellow
    Absent, //grey
}

fn evaluate_guess(secret: &str, guess: &str) -> Vec<LetterState> {
    let secret_chars: Vec<char> = secret.chars().collect();
    let guess_chars: Vec<char> = guess.chars().collect();

    let mut result: Vec<LetterState> = vec![LetterState::Absent; WORD_LENGTH];

    let mut remaining_counts = std::collections::HashMap::new();
    for i in 0..WORD_LENGTH {
        if guess_chars[i] == secret_chars[i] {
            result[i] = LetterState::Correct;
        } else {
            *remaining_counts.entry(secret_chars[i]).or_insert(0usize) += 1;
        }
    }

    for i in 0..WORD_LENGTH {
        if result[i] == LetterState::Correct {
            continue;
        }
        let g = guess_chars[i];
        if let Some(count) = remaining_counts.get_mut(&g) {
            if *count > 0 {
                result[i] = LetterState::Present;
                *count -= 1;
            } else {
                result[i] = LetterState::Absent;
            }
        } else {
            result[i] = LetterState::Absent;
        }
    }

    result
}


fn main() {
    let secret: String = {
        let mut rng: rand::prelude::ThreadRng = rng();
        WORDS.choose(&mut rng).unwrap().to_string()
    };

    eprintln!("(debug) Secret: {}", secret);
    println!("Hello, world!");
}
