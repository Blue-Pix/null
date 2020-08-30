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
          "TW_NAME" => config.screen_name = val,
          "TW_TOKEN" => config.token = val,
          _ => {},
        }
      }
    });
    Ok(config)
  }
}

#[allow(unused_must_use)]
pub fn initialize() -> Result<Config> {
  let mut config = Config::new(None)?;
  
  // in case no .env file provided or missing key
  if config.screen_name.is_empty() {
    match env::var("TW_NAME") {
      Ok(name) => config.screen_name = name,
      Err(_) => { return Err(anyhow!("Please specify TW_NAME as twitter screen_name in .env file or as environment variable.")) },
    }
  }
  if config.token.is_empty() {
    match env::var("TW_TOKEN") {
      Ok(token) => config.token = token,
      Err(_) => { return Err(anyhow!("Please specify TW_TOKEN as twitter api token in .env file or as environment variable.")) },
    }
  }

  Ok(config)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_config() {
    let expected = Config {
      screen_name: String::from("a"),
      token: String::from("b"),
    };
    let actual = Config::new(Some(".env.test")).unwrap();
    assert_eq!(expected, actual);
  }
}