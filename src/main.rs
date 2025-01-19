mod appenv;

use std::env;
use std::io;
use env_logger::{Builder, Env, Logger};
use log::{error, info, LevelFilter};
use crate::appenv::ValidEnvKey;

// Constants for error messages
const MSG_ERROR_ARGUMENTS: &str = "Incorrect arguments!";
const MSG_ERROR_FLAGS: &str = "Error: invalid flag detected!";
const MSG_ERROR_READING_INPUT: &str = "Error reading input!";
const INVALID_FLAG: &str = "--bad-flag";
const VALID_FLAG: &str = "--check-env";

fn main() {
    initialize_logger();

    let args: Vec<String> = std::env::args().collect();
    if let Err(err) = handle_args(&args) {
        error!("{}", err);
        std::process::exit(1);
    }

    if let Err(err) = read_from_stdin() {
        error!("{}", err);
        std::process::exit(1);
    }
}

/// Initializes the logger with basic configuration
fn initialize_logger() {
    Builder::new()
        .filter_level(LevelFilter::Trace)
        .init();
}

fn handle_args(args: &[String]) -> Result<(), &'static str> {
    match args.len() {
        1 => {
            info!("Count: 0");
            Ok(())
        }
        2 => {
            process_flags(args)?;
            info!("Count: {}", args.len() - 1);
            Ok(())
        }
        _ => Err(MSG_ERROR_ARGUMENTS),
    }
}

fn process_flags(args: &[String]) -> Result<(), &'static str> {
    verify_input_flags(args)?;

    if args.iter().any(|arg| arg == VALID_FLAG) {
        handle_check_env_flag()?;
    }

    Ok(())
}

fn verify_input_flags(args: &[String]) -> Result<(), &'static str> {
    if args.iter().any(|arg| arg == INVALID_FLAG) {
        return Err(MSG_ERROR_FLAGS);
    }
    Ok(())
}

/// Handles the `--check-env` flag functionality
fn handle_check_env_flag() -> Result<(), &'static str> {
    verify_environment_variables(ValidEnvKey::FOO.as_str())
}

fn verify_environment_variables(key: &str) -> Result<(), &'static str> {
    if let Some(value) = env::var_os(key) {
        let value_str = value.to_string_lossy();
        if appenv::is_env_variable_value_valid(key, &value_str).is_err() {
            return Err("Error in environment variable value.");
        }
        info!("ENV: {}={}", key, value_str);
        Ok(())
    } else {
        Err("Error reading env: does not exist!")
    }
}

fn read_from_stdin() -> Result<(), &'static str> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|_| MSG_ERROR_READING_INPUT)?;

    if ValidEnvKey::PINK.as_str().eq_ignore_ascii_case(&input.trim()) {
        info!("elephant");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::str::contains;
    use crate::appenv::ValidEnvKey;
    use crate::{MSG_ERROR_ARGUMENTS};

    /// Step 1: Basic success case
    /// Uncomment this test first and run it.
    /// It tests if the program runs without arguments and prints "Count: 0".
     #[test]
    fn test_basic_success() {
        Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .assert()
            .success()
            .stderr(contains("Count: 0"));
    }


    /// Step 2: Argument failure case
    /// Uncomment this test to see how invalid arguments cause an error.
     #[test]
    fn test_basic_failure() {
        Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .args(["invalid-arg", "one-too-many"])
            .assert()
            .failure()
            .stderr(contains(MSG_ERROR_ARGUMENTS));
    }

    /// Step 3: Testing invalid flags
    /// Uncomment this test to observe error handling for bad flags.
     #[test]
    fn test_with_bad_flags() {
        Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .args(["--bad-flag"])
            .assert()
            .failure()
            .stderr(contains("Error: invalid flag detected!"));
    }

    /// Step 4: Testing valid flags
    /// Uncomment this test to confirm the count with valid flags.
     #[test]
    fn test_with_good_flags() {
        Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .args(["--flag1"])
            .assert()
            .success()
            .stderr(contains("Count: 1"));
    }

    /// Step 5: Reading from stdin
    /// Uncomment this to test the stdin functionality.
    #[test]
    fn test_input_output() {

        let binding = vec![];
        let valid_values = crate::appenv::ENVIRONMENT_RULES
            .get(ValidEnvKey::PINK.as_str())
            .unwrap_or(&binding);

        let stderr_predicate = predicates::str::contains(valid_values.join("|"));

        Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .write_stdin(ValidEnvKey::PINK.as_str())
            .assert()
            .success()
            .stderr(stderr_predicate);
    }

    /// Step 6: Environment variable success
    /// Uncomment this to verify correct environment variable handling.
     #[test]
    fn test_environment_correct() {
        Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .args(["--check-env"])
            .env("FOO", "bar")
            .assert()
            .success()
            .stderr(contains("ENV: FOO=bar"));
    }

    /// Step 7: Environment variable failure
    /// Uncomment this to see failure when the environment variable is missing.
     #[test]
    fn test_environment_incorrect() {
        Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .unwrap()
            .args(["--check-env"])
            .assert()
            .failure()
            .stderr(contains("Error reading env: does not exist!"));
    }
}
