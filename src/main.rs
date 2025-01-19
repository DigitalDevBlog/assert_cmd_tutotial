mod appenv;
use env_logger::{Builder, Env, Logger};
use log::{error, info, LevelFilter};
use std::{env, io};

const ERROR_ARGUMENTS: &str = "Incorrect arguments!";
const ERROR_FLAGS: &str = "Error: invalid flag detected!";
const ERROR_READING_INPUT: &str = "Error reading input!";

fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Trace)
        .init();

    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => {
            println!("Count: 0");
            info!("Count: 0\n");
        }
        2 => {
            error!("{}", ERROR_ARGUMENTS);
            std::process::exit(1)
        }
        _ => {
            process_flags(&args);
            println!("Count: {}", args.len() - 1);
            info!("Count: {}\n", args.len() - 1);
        }
    }
    let mut input = String::new();
    let result = io::stdin().read_line(&mut input).unwrap_or_else(|err| {
        error!("{ERROR_READING_INPUT}: {err}");
        0
    });
    verify_standard_input(&mut input, Some(result));
}

fn process_flags(args: &[String]) {
    verify_input_flags(args);
    verify_if_env_is_needed(args);
}

fn verify_if_env_is_needed(args: &[String]) {
    if args.iter().skip(1).any(|arg| arg == "--check-env") {
        let _ = verify_environment_variables("FOO");
    }
}

fn verify_standard_input(input: &mut String, result: Option<usize>) {
    if let Some(_) = result {
        if "pink\n".eq_ignore_ascii_case(&input) {
            info!("panther!\n");
            println!("panther!");
        }
    } else {
        error!("{ERROR_READING_INPUT}");
    }
}

fn verify_environment_variables(search_for: &str) {
    match env::var_os(search_for) {
        Some(val) => {
            if appenv::is_env_variable_value_valid(search_for, &val.to_str().unwrap()).is_ok() {
                info!("ENV: {search_for}=bar\n");
                println!("ENV: {search_for}=bar");
            } else {
                error!("Error in ENV: {search_for}");
            }
        }
        None => {
            error!("Error reading env: {search_for} does not exist!");
            std::process::exit(2);
        }
    }
}

const INVALID_FLAG: &str = "--bad-flag";
fn verify_input_flags(args: &[String]) {
    if args.iter().skip(1).any(|arg| arg == crate::INVALID_FLAG) {
        error!("{ERROR_FLAGS}");
        std::process::exit(2);
    }
}
#[cfg(test)]
mod tests {
    use crate::ERROR_ARGUMENTS;
    /// Note: we are not adding 'use super::*' because we are not calling any of the production
    /// code above. This can be confusing. Running cargo test won't re-compile the above production
    /// code, because the compiler doesn't see reason for it, because we do not call any of the code
    /// above.
    ///
    /// So to have accurate test results, you must first build the code manually and then run the
    /// tests.
    use assert_cmd::Command;

    #[test]
    fn test_basic_success() -> Result<(), Box<dyn std::error::Error>> {
        let output = Command::cargo_bin(env!("CARGO_PKG_NAME"))?
            .assert()
            .success()
            .stdout(predicates::str::contains("Count: 0"));
        Ok(())
    }
    #[test]
    fn test_basic_failure() -> Result<(), Box<dyn std::error::Error>> {
        let output = Command::cargo_bin(env!("CARGO_PKG_NAME"))?
            .args(["one argument"])
            .assert()
            .failure()
            .stderr(predicates::str::contains(ERROR_ARGUMENTS));
        Ok(())
    }

    #[test]
    fn test_with_bad_flags() -> Result<(), Box<dyn std::error::Error>> {
        let output = Command::cargo_bin(env!("CARGO_PKG_NAME"))?
            .args(["--some-flag", "--bad-flag"])
            .assert()
            .failure()
            .code(2)
            .stderr(predicates::str::contains("Error: invalid flag"));
        Ok(())
    }

    #[test]
    fn test_with_good_flags() -> Result<(), Box<dyn std::error::Error>> {
        let output = Command::cargo_bin(env!("CARGO_PKG_NAME"))?
            .args(["--some-flag", "--some-other-flag"])
            .assert()
            .success()
            .stdout(predicates::str::contains("Count: 2"));
        Ok(())
    }

    #[test]
    fn test_input_output() -> Result<(), Box<dyn std::error::Error>> {
        use predicates::prelude::PredicateBooleanExt;
        let _result = Command::cargo_bin(env!("CARGO_PKG_NAME"))?
            .args(["--some-flag", "--some-other-flag"])
            .write_stdin("pink\n")
            .assert()
            .success()
            .stdout(predicates::str::contains("panther!"));
        Ok(())
    }

    #[test]
    fn test_environment_correct() -> Result<(), Box<dyn std::error::Error>> {
        let _result = Command::cargo_bin(env!("CARGO_PKG_NAME"))?
            .args(["--check-env", "--some-other-flag"])
            .env("FOO", "bar")
            .assert()
            .success()
            .stdout(predicates::str::contains("ENV: FOO=bar"));
        Ok(())
    }

    #[test]
    fn test_environment_incorrect() -> Result<(), Box<dyn std::error::Error>> {
        let _result = Command::cargo_bin(env!("CARGO_PKG_NAME"))?
            .args(["--check-env", "--some-other-flag"])
            .assert()
            .failure()
            .code(2)
            .stderr(predicates::str::contains(
                "Error reading env: FOO does not exist!",
            ));
        Ok(())
    }
}
