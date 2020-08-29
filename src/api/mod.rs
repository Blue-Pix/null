use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use url::Url;

use super::config::{Config};

#[derive(Deserialize, Debug)]
pub struct Tweet {
  pub id: u64,
  created_at: String,
  text: String,
  retweeted_status: Box<Option<Tweet>>,
  // user: User,
}

// #[derive(Deserialize, Debug)]
// pub struct User {
//   id_str: String,
//   name: String,
//   screen_name: String,
// }

#[tokio::main]
pub async fn get_tweets(config: &Config, max_id: Option<u64>) -> Result<Vec<Tweet>> {
  let mut params = vec![
    ("include_rts", "false"), 
    ("trim_user", "true"),  
    ("screen_name", &config.screen_name),
  ];
  
  let id;
  if let Some(max_id) = max_id {
    id = max_id.to_string();
    params.push(("max_id", &id));
  }

  let url = Url::parse_with_params("https://api.twitter.com/1.1/statuses/user_timeline.json", params)?;
  let client = Client::new().get(url).header("authorization", format!("Bearer {}", config.token));
  let tweets: Vec<Tweet> = client.send().await?.json().await?;
  Ok(tweets)
}