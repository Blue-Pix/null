use std::collections::HashMap;

use chrono::{Utc};
use hmacsha1::hmac_sha1;
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC, PercentEncode};
use reqwest::{Client};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

use super::config::Config;

const FRAGMENT: &AsciiSet = &NON_ALPHANUMERIC.remove(b'~').remove(b'-').remove(b'.').remove(b'_');
const OAUTH_VERSION: &str = "1.0";
const OAUTH_SIGN_METHOD: &str = "HMAC-SHA1";

#[derive(Debug)]
pub struct RequstToken {
  oauth_token: String,
  oauth_token_secret: String,
  oauth_callback_confirmed: String,
}

impl RequstToken {
  fn from_response(response: String) -> Self {
    let mut map = HashMap::new();
    response
      .split('&')
      .collect::<Vec<_>>()
      .into_iter()
      .for_each(|pair| {
        let v = pair.split('=').collect::<Vec<_>>();
        map.insert(v[0], v[1]);
      });

    Self {
      oauth_token: map.get("oauth_token").unwrap().to_string(),
      oauth_token_secret: map.get("oauth_token_secret").unwrap().to_string(),
      oauth_callback_confirmed: map.get("oauth_callback_confirmed").unwrap().to_string(),
    }
  }
}

pub async fn get_request_token(config: &Config) -> RequstToken {
  let url = "https://api.twitter.com/oauth/request_token";
  let mut headers = HeaderMap::new();
  headers.insert(AUTHORIZATION, create_get_request_token_header(config, url).parse().unwrap());
  headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));

  let client = Client::new();
  let res = client.post(url).headers(headers).send().await.unwrap().text().await.unwrap();
  RequstToken::from_response(res)
}

fn create_get_request_token_header(config: &Config, url: &str) -> String {
  let timestamp = Utc::now().timestamp().to_string();
  let callback = "";

  let mut params: HashMap<&str, &str> = HashMap::new();
  params.insert("oauth_consumer_key", &config.api_key);
  params.insert("oauth_nonce", &timestamp);
  params.insert("oauth_signature_method", OAUTH_SIGN_METHOD);
  params.insert("oauth_timestamp", &timestamp);
  params.insert("oauth_version", OAUTH_VERSION);
  params.insert("oauth_callback", callback);
  
  let signature = create_oauth_signature("POST", url, &config.api_secret, "", &params);
  format!(
    r#"OAuth oauth_nonce="{}", oauth_callback="{}", oauth_signature_method="{}", oauth_timestamp="{}", oauth_consumer_key="{}", oauth_signature="{}", oauth_version="{}""#,
    encode(params.get("oauth_nonce").unwrap()),
    encode(params.get("oauth_callback").unwrap()),
    encode(params.get("oauth_signature_method").unwrap()),
    encode(params.get("oauth_timestamp").unwrap()),
    encode(params.get("oauth_consumer_key").unwrap()),
    encode(&signature),
    encode(params.get("oauth_version").unwrap()),
  )
}

pub fn create_oauth1_header(config: &Config, url: &str) -> String {
  let timestamp = Utc::now().timestamp().to_string();

  let mut params: HashMap<&str, &str> = HashMap::new();
  params.insert("oauth_consumer_key", &config.api_key);
  params.insert("oauth_nonce", &timestamp);
  params.insert("oauth_signature_method", OAUTH_SIGN_METHOD);
  params.insert("oauth_timestamp", &timestamp);
  params.insert("oauth_version", OAUTH_VERSION);
  params.insert("oauth_token", &config.access_token);
  
  let signature = create_oauth_signature("POST", url, &config.api_secret, &config.access_secret, &params);
  format!(
    r#"OAuth oauth_nonce="{}", oauth_signature_method="{}", oauth_timestamp="{}", oauth_consumer_key="{}", oauth_signature="{}", oauth_version="{}", oauth_token="{}""#,
    encode(params.get("oauth_nonce").unwrap()),
    encode(params.get("oauth_signature_method").unwrap()),
    encode(params.get("oauth_timestamp").unwrap()),
    encode(params.get("oauth_consumer_key").unwrap()),
    encode(&signature),
    encode(params.get("oauth_version").unwrap()),
    encode(&config.access_token),
  )
}

fn create_oauth_signature(
  http_method: &str,
  url: &str,
  oauth_consumer_secret: &str,
  oauth_token_secret: &str,
  params: &HashMap<&str, &str>,
) -> String {
  let key = create_signature_key(oauth_consumer_secret, oauth_token_secret);
  let data = create_signature_data(http_method, url, params);
  let hash = hmac_sha1(key.as_bytes(), data.as_bytes());
  base64::encode(&hash)
}

fn create_signature_key(
  oauth_consumer_secret: &str,
  oauth_token_secret: &str
) -> String {
  let oauth_consumer_secret = encode(oauth_consumer_secret);
  let oauth_token_secret = encode(oauth_token_secret);
  format!("{}&{}", oauth_consumer_secret, oauth_token_secret)
}

fn create_signature_data(
  http_method: &str,
  url: &str,
  params: &HashMap<&str, &str>
) -> String {
  let mut params: Vec<(&&str, &&str)> = params.into_iter().collect();
  params.sort();
  let params = params
    .into_iter()
    .map(|(k, v)| format!("{}={}", k, v))
    .collect::<Vec<String>>()
    .join("&");

  let http_method = encode(http_method);
  let url = encode(url);
  let params = encode(&params);

  format!("{}&{}&{}", http_method, url, params)
}

fn encode(input: &str) -> PercentEncode {
  utf8_percent_encode(input, FRAGMENT)
}