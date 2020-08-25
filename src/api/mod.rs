use std::env;

use error_chain::error_chain;
use reqwest::Client;
use serde::Deserialize;
use url::Url;

error_chain! {
  foreign_links {
    HttpRequest(reqwest::Error);
    Url(url::ParseError);
    Var(env::VarError);
  }
}

#[derive(Deserialize, Debug)]
pub struct Tweet {
  id_str: String,
  created_at: String,
  text: String,
  retweeted_status: Box<Option<Tweet>>,
}

#[tokio::main]
pub async fn get_tweets(screen_name: &str) -> Result<Vec<Tweet>> {
  let url = Url::parse_with_params(
    "https://api.twitter.com/1.1/statuses/user_timeline.json",
    &[("screen_name", screen_name), ("include_rts", "false")],
  )?;
  let client = Client::new().get(url).header("authorization", format!("Bearer {}", env::var("TOKEN")?));
  let tweets: Vec<Tweet> = client.send().await?.json().await?;
  Ok(tweets)
}
