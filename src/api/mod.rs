use anyhow::Result;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;
use url::Url;

use super::config::{Config};
use super::oauth;

#[derive(Deserialize, Debug)]
pub struct Tweet {
  pub id: u64,
  created_at: String,
  pub text: String,
  retweeted_status: Box<Option<Tweet>>,
  // user: User,
}

// #[derive(Deserialize, Debug)]
// pub struct User {
//   id_str: String,
//   name: String,
//   screen_name: String,
// }

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

pub async fn delete_tweet(config: &Config, id: u64) -> Result<Tweet> {
  let url = format!("https://api.twitter.com/1.1/statuses/destroy/{}.json", id.to_string());
  let header = oauth::create_oauth1_header(&url);
  let mut headers = HeaderMap::new();
  headers.insert(AUTHORIZATION, header.parse().unwrap());
  headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));

  let url = Url::parse(&url)?;
  let client = Client::new().post(url).headers(headers);
  let tweet: Tweet = client.send().await?.json().await?;
  Ok(tweet)
}