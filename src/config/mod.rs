use std::env;
use std::fs;
use std::str;

use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq)]
pub struct Config {
  pub screen_name: String,
  pub token: String,
}

const ENV_FILE: &str = ".env"; 
const NAME_KEY: &str = "TW_NAME";
const TOKEN_KEY: &str = "TW_TOKEN";

impl Config {
  pub fn new(filename: Option<&str>) -> Result<Self> {
    let mut config = Self { screen_name: String::new(), token: String::new() };
    let content = match filename {
      // [ToDo] handle error around file
      Some(filename) => fs::read_to_string(filename)?,
      None => fs::read_to_string(ENV_FILE)?
    };
    content.lines().for_each(|line| {
      let entries: Vec<_> = line.split("=").map(str::trim).collect();
      if entries.len() == 2 {
        let key = entries[0].trim();
        let val = entries[1].trim().to_string();
        match key {
          NAME_KEY => config.screen_name = val,
          TOKEN_KEY => config.token = val,
          _ => {},
        }
      }
    });
    config.overwrite_with_env_vars()?.validate()?;
    
    Ok(config)
  }

  fn overwrite_with_env_vars(&mut self) -> Result<&Self> {
    match env::var(NAME_KEY) {
      Ok(name) => self.screen_name = name,
      Err(_) => {},
    }
    match env::var(TOKEN_KEY) {
      Ok(token) => self.token = token,
      Err(_) => {},
    }
    Ok(self)
  }

  fn validate(&self) -> Result<()> {
    if self.screen_name.is_empty() {
      return Err(anyhow!("Please specify {} as twitter screen_name in .env file or as environment variable.", NAME_KEY))
    }
    if self.token.is_empty() {
      return Err(anyhow!("Please specify {} as twitter api token in .env file or as environment variable.", TOKEN_KEY))
    }
    Ok(())
  }
}

///
/// should run as `cargo test -- --test-threads=1`
/// because of confict of environment variables.
/// 
#[cfg(test)]
mod tests {
  use super::*;

  fn setup() {
    env::remove_var(NAME_KEY);
    env::remove_var(TOKEN_KEY);
  }

  #[test]
  fn initialize_config_with_file() {
    setup();
    let expected = Config {
      screen_name: String::from("a"),
      token: String::from("b"),
    };
    let actual = Config::new(Some(".env.test")).unwrap();
    assert_eq!(expected, actual);
  }

  #[test]
  fn initialize_config_with_env_vars() {
    setup();
    env::set_var(NAME_KEY, "c");
    env::set_var(TOKEN_KEY, "d");

    let expected = Config {
      screen_name: String::from("c"),
      token: String::from("d"),
    };
    let actual = Config::new(Some(".env.test.none")).unwrap();
    assert_eq!(expected, actual);
  }

  #[test]
  fn overwrite_config_with_env_vars() {
    setup();
    env::set_var(NAME_KEY, "c");
    env::set_var(TOKEN_KEY, "d");

    let expected = Config {
      screen_name: String::from("c"),
      token: String::from("d"),
    };
    let actual = Config::new(Some(".env.test")).unwrap();
    assert_eq!(expected, actual);
  }

  #[test]
  #[should_panic(expected = "Please specify TW_NAME as twitter screen_name in .env file or as environment variable.")]
  fn missing_screen_name() {
    setup();
    env::set_var(TOKEN_KEY, "d");

    Config::new(Some(".env.test.none")).unwrap();
  }

  #[test]
  #[should_panic(expected = "Please specify TW_TOKEN as twitter api token in .env file or as environment variable.")]
  fn missing_token() {
    setup();
    env::set_var(NAME_KEY, "c");

    Config::new(Some(".env.test.none")).unwrap();
  }
}