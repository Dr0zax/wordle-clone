use std::io::{self, Write};
use rand::seq::{IndexedRandom};
use rand::rng;
use serde::Deserialize;

const WORD_LENGTH: usize = 5;
const MAX_GUESSES: usize = 6;

#[derive(Deserialize)]
struct WordList {
    words: Vec<String>,
}

fn load_words() -> Vec<String> {
    let data = include_str!("word_list.json");
    let word_list: WordList = serde_json::from_str(data).expect("Failed to parse word_list.json");
    word_list.words
}


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

fn print_feedback(guess: &str, feedback: &[LetterState]) {
    for (c, state) in guess.chars().zip(feedback.iter()) {
        match state {
            LetterState::Correct => print!("\x1b[42m{}\x1b[0m", c), // Green background
            LetterState::Present => print!("\x1b[43m{}\x1b[0m", c), // Yellow background
            LetterState::Absent => print!("\x1b[47m{}\x1b[0m", c),  // Grey background
        }
    }
    println!();
}

fn read_guess() -> io::Result<String> {
    print!("Enter your guess: ");
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_lowercase())
}

fn is_valid_guess(guess: &str) -> bool {
    return guess.len() == WORD_LENGTH && guess.chars().all(|c| c.is_ascii_lowercase());
}


fn main() {
    let words = load_words();
    let secret: String = {
        let mut rng: rand::prelude::ThreadRng = rng();
        words.choose(&mut rng).unwrap().to_string()
    };

    // eprintln!("(debug) Secret: {}", secret);

    println!("Welcome to a word guessing game that is legally distinct and not related in any way to Wordle!");

    for attempt in 1..=MAX_GUESSES {
        let guess = loop {
            match read_guess() {
                Ok(g) if is_valid_guess(&g) => break g,
                _ => println!("Invalid guess. Please enter a {}-letter word.", WORD_LENGTH),
            }
        };

        let feedback = evaluate_guess(&secret, &guess);
        print_feedback(&guess, &feedback);

        if feedback.iter().all(|&s| s == LetterState::Correct) {
            println!("Congratulations! You've guessed the word in {} attempts!", attempt);
            return;
        }
    }
    println!("Sorry, you've used all your attempts. The secret word was: {}", secret);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_guess() {
        let secret = "apple";
        let guess = "apric";
        let feedback = evaluate_guess(secret, guess);
        assert_eq!(feedback, vec![
            LetterState::Correct,
            LetterState::Correct,
            LetterState::Absent,
            LetterState::Absent,
            LetterState::Absent,
        ]);

        let guess2 = "peach";
        let feedback2 = evaluate_guess(secret, guess2);
        assert_eq!(feedback2, vec![
            LetterState::Present,
            LetterState::Present,
            LetterState::Present,
            LetterState::Absent,
            LetterState::Absent,
        ]);

        let guess3 = "apple";
        let feedback3 = evaluate_guess(secret, guess3);
        assert_eq!(feedback3, vec![
            LetterState::Correct,
            LetterState::Correct,
            LetterState::Correct,
            LetterState::Correct,
            LetterState::Correct,
        ]);
    } 

    #[test]
    fn test_is_valid_guess() {
        assert!(is_valid_guess("apple"));
        assert!(!is_valid_guess("appl")); // too short
        assert!(!is_valid_guess("apples")); // too long
        assert!(!is_valid_guess("AppLe")); // uppercase letters
        assert!(!is_valid_guess("appl3")); // non-letter character
    }
}