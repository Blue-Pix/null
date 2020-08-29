use std::collections::HashMap;
use std::env;
use std::fs;
use std::str;

use anyhow::Result;

#[derive(Debug)]
pub struct Config {
  pub screen_name: String,
  pub token: String,
}

#[allow(unused_must_use)]
pub fn initialize() -> Result<Config> {
  let mut map = HashMap::new();
  // [ToDo] handle error around file
  let content = fs::read_to_string(".env")?;
  content.lines().for_each(|line| {
    let entries: Vec<_> = line.split("=").map(str::trim).collect();
    if entries.len() == 2 {
      let key = entries[0].trim();
      let val = entries[1].trim().to_string();
      map.insert(key, val);
    }
  });
  // in case no .env file provided or missing key
  match env::var("TW_NAME") {
    Ok(name) => { map.entry("TW_NAME").or_insert(name); },
    Err(_) => {},
  }
  match env::var("TW_TOKEN") {
    Ok(token) => { map.entry("TW_TOKEN").or_insert(token); },
    Err(_) => {},
  }

  Ok(Config{
    screen_name: map.get("TW_NAME").expect("Please specify TW_NAME as twitter screen_name in .env file or as environment variable.").to_string(),
    token: map.get("TW_TOKEN").expect("Please specify TW_TOKEN as twitter api token in .env file or as environment variable.").to_string(),
  })
}