use std::{collections::HashMap, env::Vars};

use regex::Regex;

const SECRET_MIN_SIZE: u8 = 5;
const SECRET_REPLACEMENT: &str = "[secure]";

/// Identify candidates in std::env::vars() that contain sensitive information and return a callback function that, given an input, masks these sensitive values and returns the modified output.
///
/// # Panics
///
/// It is very unlikely to panic because it constructs the regex pattern for replacing variables by escaping characters. However, if there is an issue during regex pattern construction, it will panic.
///
/// # Example
/// ```
/// let hide_sensitive_fn = hide_sensitive(std::env::vars());
///
/// let result = assert_eq!(hide_sensitive_fn("My API token is 12345"), "My API token is [secure]");
/// ```
pub fn hide_sensitive(env_vars: Vars) -> impl Fn(&str) -> String {
    let re = Regex::new(r"(?i)token|password|credential|secret|private").unwrap();

    let sensitive_vars: HashMap<String, String> = env_vars
        .filter(|(env_var, value)| {
            if env_var == "GOPRIVATE" {
                return false;
            }
            re.is_match(env_var) && value.trim().chars().count() >= SECRET_MIN_SIZE as usize
        })
        .collect();

    let pattern = sensitive_vars
        .values()
        .map(|v| regex::escape(v))
        .collect::<Vec<String>>()
        .join("|");

    let regexp = Regex::new(&pattern).expect("should be a valid regex");

    move |input: &str| -> String { regexp.replace_all(input, SECRET_REPLACEMENT).to_string() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hide_sensitive() {
        std::env::set_var("API_TOKEN", "12345");
        std::env::set_var("USER_CREDENTIAL", "test123");
        std::env::set_var("USER_PASSWORD", "secret_password");
        std::env::set_var(
            "DATABASE_URL",
            "postgres://user:password@localhost:5432/dbname",
        );
        std::env::set_var("GOPRIVATE", "sensitive_data");

        let hide_sensitive_fn = hide_sensitive(std::env::vars());

        let input = "My API token is 12345 and my password is secret_password. USER_CREDENTIAL test123 is hidden GOPRIVATE should be ignored, and DATABASE_URL postgres://user:password@localhost:5432/dbname is safe.";
        let expected_output = "My API token is [secure] and my password is [secure]. USER_CREDENTIAL [secure] is hidden GOPRIVATE should be ignored, and DATABASE_URL postgres://user:password@localhost:5432/dbname is safe.";

        let result = hide_sensitive_fn(input);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_case_insensitivity() {
        std::env::set_var("API_token", "12345");
        std::env::set_var("USER_password", "secret_password");

        let hide_sensitive_fn = hide_sensitive(std::env::vars());

        let input = "My API token is 12345 and my password is secret_password.";
        let expected_output = "My API token is [secure] and my password is [secure].";

        let result = hide_sensitive_fn(input);
        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_special_characters_in_sensitive_values() {
        std::env::set_var("USER_PASSWORD", "p@ssw*rd!");
        std::env::set_var("API_TOKEN", "12345");

        let hide_sensitive_fn = hide_sensitive(std::env::vars());

        let input = "Sensitive value: p@ssw*rd! should be hidden, API_TOKEN is 12345.";
        let expected_output = "Sensitive value: [secure] should be hidden, API_TOKEN is [secure].";

        let result = hide_sensitive_fn(input);
        assert_eq!(result, expected_output);
    }
}
