use std::io::{self};
#[macro_use] extern crate log;
extern crate thiserror;
use thiserror::Error;
use rand::Rng;

/// A type to represent the output of validate_input
pub type ValidationResult = std::result::Result<(), ValidationError>;
/// Max number of attempts to guess
const MAX_GUESSES: u32 = 20;
/// Max length of the color code
const MAX_LENGTH: u32 = 10;
/// Min length of the color code
const MIN_LENGTH: u32 = 4;
/// Max number of symbols (colors to choose)
const MAX_SYMBOLS: u8 = 20;
/// Min number of symbols (colors to choose)
const MIN_SYMBOLS: u8 = 2;

/// Finds all patterns in a String, returning the indexes in Vec<usize>
macro_rules! findall {
    ($x: ident, $y: ident) => {
        $x.match_indices($y).map(|(idx, _)| idx).collect()
    }
}

#[derive(Error, Debug)]
/// Custom error to represent all possible errors that might arise parsing user input
pub enum ValidationError {

    #[error("Parse error on user input")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Input does not respect the rule `{0}`")]
    Invalid(String)
}

/// Validates user input: number of attempts
fn validate_attempts(number: u32) -> ValidationResult {

    if number > MAX_GUESSES {
        return Err(ValidationError::Invalid(
            format!("Exceeded the max number of attempts allowed ({})", MAX_GUESSES))
        )
    }
    else if number == 0 {
        return Err(ValidationError::Invalid(format!("0 is not allowed!")))
    }
    Ok(())
}

/// Validates user input: length of the code that will be created
/// Example, test some input:
/// ```
/// # type ValidationResult = std::result::Result<(), crate::mastermind::ValidationError>;
/// let result = mastermind::validate_length(3).expect_err("will fail");
/// assert_eq!(
///     "Input does not respect the rule `Length of the code should be between 4 and 10`",
///     format!("{}", result)
/// );
/// mastermind::validate_length(4).expect("Won't fail");
/// mastermind::validate_length(10).expect("Won't fail");
/// mastermind::validate_length(11).expect_err("This should fail");
/// ```
pub fn validate_length(length: u32) -> ValidationResult {

    if (MIN_LENGTH > length) || (length > MAX_LENGTH) {
        return Err(ValidationError::Invalid(
            format!("Length of the code should be between {} and {}", MIN_LENGTH, MAX_LENGTH))
        )
    }
    Ok(())
}

/// Validates user input: max/min number of different colors on the code
/// Example, test some input:
/// ```
/// # type ValidationResult = std::result::Result<(), crate::mastermind::ValidationError>;
/// let result = mastermind::validate_nsymbols(1).expect_err("will fail");
/// assert_eq!(
///     "Input does not respect the rule `The number of symbols should be between 2 and 20`",
///     format!("{}", result)
/// );
/// mastermind::validate_nsymbols(2).expect("Won't fail");
/// mastermind::validate_nsymbols(20).expect("Won't fail");
/// mastermind::validate_nsymbols(21).expect_err("This should fail");
/// ```
pub fn validate_nsymbols(length: u8) -> ValidationResult {

    if (MIN_SYMBOLS > length) || (length > MAX_SYMBOLS) {
        return Err(ValidationError::Invalid(
            format!("The number of symbols should be between {} and {}", MIN_SYMBOLS, MAX_SYMBOLS))
        )
    }
    Ok(())
}

/// Creates a random string of `length` using up to `n_symbols` different symbols
fn create_secret_code(length: u32, n_symbols: u8) -> String {

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRST";
    let mut rng = rand::thread_rng();

    info!("Creating random number");
    let secret: String = (0..length).map(|_| {
            let idx: usize = usize::from(rng.gen_range(0u8, n_symbols));
            CHARSET[idx] as char
        }).collect();
    debug!("Secret code chosen: '{}'", &secret);
    secret
}

/// Reads a single line (user guess) from standard input
fn get_user_guess() -> String {
    println!("Try to guess my secret code: ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)
        .expect("Had problems reading user input!");
    let len = buffer.trim_end_matches(&['\r', '\n'][..]).len();
    buffer.truncate(len);
    buffer
}

/// Compares the secret code with the user guess. If they do not match,
/// return as error the sequence of 'X', 'O' where:
///   * 'X' is an exact match (symbol and position)
///   * 'O' matches a symbol, but not a position
fn check_user_guess(secret: &String, guess: &String) -> String {

    let mut result = String::new();
    let mut results = (0u8, 0u8, 0u8);  // number of X, O, -
    for (index, c) in guess.char_indices() {
        let indexes: Vec<usize> = findall!(secret, c);
        if indexes.is_empty() {
            results.2 += 1
        } else {
            if indexes.into_iter().find(|&x| x == index).is_some() {
                results.0 += 1
            } else {
                results.1 += 1
            }
        }
    }
    let mut index: usize = 0;
    for _idx in 0..results.0 {
        result.insert(index, 'X');
        index += 1;
    }
    for _idx in 0..results.1 {
        result.insert(index, 'O');
        index += 1;
    }
    for _idx in 0..results.2 {
        result.insert(index, '-');
        index += 1;
    }
    result
}

/// Main application loop, generates the secret code and allows the user
/// to input guesses, calculating and printing the result
pub fn run(attempts: u32, length: u32, n_symbols: u8) -> Result<(), &'static str> {

    validate_attempts(attempts).expect("Validation error: incorrect attempts");
    validate_length(length).expect("Validation error: incorrect code length");
    validate_nsymbols(n_symbols).expect("Validation error: incorrect number of symbols");

    let secret = create_secret_code(length, n_symbols);
    let mut expected = String::with_capacity(length as usize);
    for _ in 0..length {
        expected.push('X');
    }
    println!("{}", secret);
    let mut guesses: u32 = 1;
    loop {
        let guess = get_user_guess();
        debug!("User guessed: '{}'", guess);
        let guess_result = check_user_guess(&secret, &guess);
        if guess_result == expected {
            return Ok(())
        }
        println!("Nope: {}", guess_result);
        if guesses >= attempts {
            return Err("Max number of attempts reached");
        }
        debug!("User guess did not match: retry");
        guesses += 1;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_validate_attempts() {
        assert_eq!(validate_attempts(20).expect("Should work"), ());
        let result = validate_attempts(21).expect_err("will fail");
        assert_eq!(
            "Input does not respect the rule `Exceeded the max number of attempts allowed (20)`",
            format!("{}", result)
        );
        let result = validate_attempts(0).expect_err("will fail");
        assert_eq!(
            "Input does not respect the rule `0 is not allowed!`",
            format!("{}", result)
        );
    }

    #[test]
    fn test_secret_code() {
        let secret = create_secret_code(5, 1);
        assert_eq!(secret.len(), 5);
        for c in secret.chars() {
            assert_eq!('\u{0041}', c);
        }
        // with only two symbols to create the secret code, each char should be 'A' or 'B'
        let secret = create_secret_code(5, 2);
        assert_eq!(secret.len(), 5);
        for c in secret.chars() {
            assert!('\u{0041}' == c || '\u{0042}' == c);
        }
    }

    #[test]
    fn test_check_user_guess() {
        assert_eq!("----", check_user_guess(&String::from("AAAA"), &String::from("BBBB")));
        assert_eq!("XX--", check_user_guess(&String::from("ABBA"), &String::from("ACCA")));
        assert_eq!("OOOO", check_user_guess(&String::from("AABB"), &String::from("BBAA")));
        assert_eq!("XXXX", check_user_guess(&String::from("ABCD"), &String::from("ABCD")));
    }
}