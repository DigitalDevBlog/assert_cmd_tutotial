use log::error;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub enum ValidEnvKey {
    FOO,
    PINK,
}

impl ValidEnvKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            ValidEnvKey::FOO => "FOO",
            ValidEnvKey::PINK => "PINK",
        }
    }

    /// Returns all valid keys as a slice.
    pub fn all() -> &'static [ValidEnvKey] {
        &[ValidEnvKey::FOO, ValidEnvKey::PINK]
    }
}

/// Represents a single environmental validation rule.
#[derive(Debug)]
pub struct ValidEnvRule<'a> {
    pub key: &'a str,
    pub valid_values: &'a [&'a str],
}

/// Centralized environment validation rules.
pub static ENVIRONMENT_RULES: Lazy<HashMap<&str, Vec<&str>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(ValidEnvKey::FOO.as_str(), vec!["bar"]);
    map.insert(ValidEnvKey::PINK.as_str(), vec!["elephant"]);
    map
});

/// Wrapper for environment variable
#[derive(Debug)]
struct ValidEnvKeyValue<'a> {
    pub key: &'a str,
    pub valid_values: &'a [&'a str],
}

fn is_valid_env_variable(key: &str) -> Result<(), String> {
    if ENVIRONMENT_RULES.contains_key(key) {
        Ok(())
    } else {
        error!("Invalid environment variable: {key}");
        Err(format!("Invalid environment variable: {key}"))
    }
}

pub(crate) fn is_env_variable_value_valid(key: &str, value: &str) -> Result<(), String> {
    if key.is_empty() || value.is_empty() {
        error!("Key or value cannot be empty");
        return Err("Key or value cannot be empty".to_string());
    }
    match ENVIRONMENT_RULES.get(key) {
        Some(valid_values) if valid_values.contains(&value) => Ok(()),
        _ => {
            error!("Invalid value for environment variable: {key}, Found value: {value}, expected on of value: {:?}", ENVIRONMENT_RULES.get(key).unwrap_or(&vec![]));
            Err(format!("Invalid value for environment variable: {key}, Found value: {value}, expected on of value: {:?}", ENVIRONMENT_RULES.get(key).unwrap_or(&vec![])))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_env_valid_variable() {
        assert_eq!(true, is_valid_env_variable(ValidEnvKey::FOO.as_str()).is_ok());
    }

    #[test]
    fn test_env_variable_value_empty_value() {
        assert_eq!(
            is_env_variable_value_valid(ValidEnvKey::FOO.as_str(), ""),
            Err("Key or value cannot be empty".to_string())
        );
    }
    #[test]
    fn test_env_variable_value_empty_key() {
        assert_eq!(
            is_env_variable_value_valid("", "bar"),
            Err("Key or value cannot be empty".to_string())
        );
    }
    #[test]
    fn test_env_variable_value_valid_key_and_value() {
        assert!(is_env_variable_value_valid(ValidEnvKey::FOO.as_str(), "bar").is_ok());
    }

    #[test]
    fn test_env_variable_value_invalid_value_for_key() {
        assert_eq!(
            is_env_variable_value_valid(ValidEnvKey::FOO.as_str(), "baz"),
            Err("Invalid value for environment variable: FOO, Found value: baz, expected on of value: [\"bar\"]".to_string())
        );
    }
}
