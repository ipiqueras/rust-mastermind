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
fn create_secret_code(length: u32, n_symbols: u8) {

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRST";
    let mut rng = rand::thread_rng();

    info!("Creating random number");
    let secret: String = (0..length)
        .map(|_| {
            let idx: usize = usize::from(rng.gen_range(0u8, n_symbols));
            CHARSET[idx] as char
        })
        .collect();
    debug!("Secret code chosen: '{}'", &secret);
    //&secret
}


/// Main application loop, generates the secret code and allows the user
/// to input guesses, calculating and printing the result
pub fn run(attempts: u32, length: u32, n_symbols: u8) -> Result<(), String> {

    validate_attempts(attempts).expect("Validation error: incorrect attempts");
    validate_length(length).expect("Validation error: incorrect code length");
    validate_nsymbols(n_symbols).expect("Validation error: incorrect number of symbols");
    create_secret_code(length, n_symbols);
    Ok(())
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
}