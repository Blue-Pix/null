use std::env;
use std::fs;
use std::io::ErrorKind;
use std::str;

use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq)]
pub struct Config {
  pub screen_name: String,
  pub token: String,
  pub api_key: String,
  pub api_secret: String,
  pub access_token: String,
  pub access_secret: String,
}

const ENV_FILE: &str = ".env"; 
const NAME_KEY: &str = "TW_NAME";
const TOKEN_KEY: &str = "TW_TOKEN";
const API_KEY: &str = "API_KEY";
const API_SECRET: &str = "API_SECRET";
const ACCESS_TOKEN: &str = "ACCESS_TOKEN";
const ACCESS_SECRET: &str = "ACCESS_SECRET";

impl Config {
  pub fn new(filename: Option<&str>) -> Result<Self> {
    let mut config = Self { 
      screen_name: String::new(), 
      token: String::new(),
      api_key: String::new(),
      api_secret: String::new(),
      access_token: String::new(),
      access_secret: String::new(),
    };
    let content = match filename {
      Some(filename) => read_env_file(filename)?,
      None => read_env_file(ENV_FILE)?
    };
    content.lines().for_each(|line| {
      let entries: Vec<_> = line.split("=").map(str::trim).collect();
      if entries.len() == 2 {
        let key = entries[0].trim();
        let val = entries[1].trim().to_string();
        match key {
          NAME_KEY => config.screen_name = val,
          TOKEN_KEY => config.token = val,
          API_KEY => config.api_key = val,
          API_SECRET => config.api_secret = val,
          ACCESS_TOKEN => config.access_token = val,
          ACCESS_SECRET => config.access_secret = val,
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
    match env::var(API_KEY) {
      Ok(api_key) => self.api_key = api_key,
      Err(_) => {},
    }
    match env::var(API_SECRET) {
      Ok(api_secret) => self.api_secret = api_secret,
      Err(_) => {},
    }
    match env::var(ACCESS_TOKEN) {
      Ok(access_token) => self.access_token = access_token,
      Err(_) => {},
    }
    match env::var(ACCESS_SECRET) {
      Ok(access_secret) => self.access_secret = access_secret,
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
    if self.api_key.is_empty() {
      return Err(anyhow!("API_KEY is missing."));
    }
    if self.api_secret.is_empty() {
      return Err(anyhow!("API_SECRET is missing."));
    }
    if self.access_token.is_empty() {
      return Err(anyhow!("ACCESS_TOKEN is missing."));
    }
    if self.access_secret.is_empty() {
      return Err(anyhow!("ACCESS_SECRET is missing."));
    }
    Ok(())
  }
}

fn read_env_file(filename: &str) -> Result<String> {
  match fs::read_to_string(filename) {
    Ok(content) => return Ok(content),
    Err(ref e) if e.kind() == ErrorKind::NotFound => {
      // do nothing
      return Ok(String::new());
    },
    Err(e) => return Err(anyhow!(e))
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
      api_key: String::from("z"),
      api_secret: String::from("z"),
      access_token: String::from("z"),
      access_secret: String::from("z"),
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
      api_key: String::from("z"),
      api_secret: String::from("z"),
      access_token: String::from("z"),
      access_secret: String::from("z"),
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
      api_key: String::from("z"),
      api_secret: String::from("z"),
      access_token: String::from("z"),
      access_secret: String::from("z"),
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

  #[test]
  fn missing_env_file() {
    setup();
    env::set_var(NAME_KEY, "c");
    env::set_var(TOKEN_KEY, "d");

    let expected = Config {
      screen_name: String::from("c"),
      token: String::from("d"),
      api_key: String::from("z"),
      api_secret: String::from("z"),
      access_token: String::from("z"),
      access_secret: String::from("z"),
    };
    let actual = Config::new(Some("no_such_file")).unwrap();
    assert_eq!(expected, actual);
  }

  #[test]
  #[should_panic(expected = "Is a directory (os error 21)")]
  fn unexpected_error_when_read_env_file() {
    setup();
    Config::new(Some("target")).unwrap();
  }
}