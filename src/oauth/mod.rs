use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use chrono::{Utc};
use std::collections::HashMap;
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use reqwest::{Client};

const FRAGMENT: &AsciiSet = &NON_ALPHANUMERIC.remove(b'*').remove(b'-').remove(b'.').remove(b'_');

#[derive(Debug, Clone)]
pub struct RequstToken {
  oauth_token: String,
  oauth_token_secret: String,
  oauth_callback_confirmed: String,
}

#[tokio::main]
pub async fn get_request_token() -> String {
  let endpoint = "https://api.twitter.com/oauth/request_token";
  let header_auth = get_request_header(endpoint);
  let mut headers = HeaderMap::new();
  headers.insert(AUTHORIZATION, header_auth.parse().unwrap());
  headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));

  let client = Client::new();
  let res = client
    .post(endpoint)
    .headers(headers)
    .send()
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

  res

  // let res_values = res.split('&').map(|s| s.split('='))

  // RequstToken {
  //   oauth_token: "".to_string(),
  //   oauth_token_secret: "".to_string(),
  //   oauth_callback_confirmed: "".to_string(),
  // }
}

fn get_request_header(endpoint: &str) -> String {
  let oauth_consumer_key = &from_env("CONSUMER_KEY");
  let oauth_consumer_secret = &from_env("CONSUMER_SECRET");
  let oauth_nonce: &str = &format!("nonce{}", Utc::now().timestamp());
  let oauth_callback = "";
  let oauth_signature_method = &"HMAC-SHA1";
  let timestamp = &format!("{}", Utc::now().timestamp());
  let oauth_version = "1.0";

  let mut map = HashMap::new();
  map.insert("oauth_nonce", oauth_nonce);
  map.insert("oauth_callback", oauth_callback);
  map.insert("oauth_signature_method", oauth_signature_method);
  map.insert("oauth_timestamp", timestamp);
  map.insert("oauth_version", oauth_version);
  map.insert("oauth_consumer_key", oauth_consumer_key);

  let oauth_signature = &create_oauth_signature(
    "POST",
    &endpoint,
    oauth_consumer_secret,
    "",
    &map
  );

  format!(
    r#"OAuth oauth_nonce="{}", oauth_callback="{}", oauth_signature_method="{}", oauth_timestamp="{}", oauth_consumer_key="{}", oauth_signature="{}", oauth_version="{}""#,
    utf8_percent_encode(oauth_nonce, FRAGMENT),
    utf8_percent_encode(oauth_callback, FRAGMENT),
    utf8_percent_encode(oauth_signature_method, FRAGMENT),
    utf8_percent_encode(timestamp, FRAGMENT),
    utf8_percent_encode(oauth_consumer_key, FRAGMENT),
    utf8_percent_encode(oauth_signature, FRAGMENT),
    utf8_percent_encode(oauth_version, FRAGMENT),
  )
}

fn from_env(name: &str) -> String {
  match std::env::var(name) {
      Ok(val) => val,
      Err(err) => {
          println!("{}: {}", err, name);
          std::process::exit(1);
      }
  }
}


fn create_oauth_signature(
  http_method: &str,
  endpoint: &str,
  oauth_consumer_secret: &str,
  oauth_token_secret: &str,
  params: &HashMap<&str, &str>,
) -> String {
  let oauth_consumer_secret_encoded = utf8_percent_encode(oauth_consumer_secret, FRAGMENT);
  let oauth_token_secret_encoded = utf8_percent_encode(oauth_token_secret, FRAGMENT);
  let key = format!("{}&{}", oauth_consumer_secret_encoded, oauth_token_secret_encoded);

  let mut param = String::new();
  let mut params: Vec<(&&str, &&str)> = params.into_iter().collect();
  params.sort();

  let param = params
    .into_iter()
    .map(|(k, v)| {
      format!(
        "{}={}",
        utf8_percent_encode(k, FRAGMENT),
        utf8_percent_encode(v, FRAGMENT),
      )
    })
    .collect::<Vec<String>>()
    .join("&");

  let http_method_encoded = utf8_percent_encode(http_method, FRAGMENT);
  let endpoint_encoded = utf8_percent_encode(endpoint, FRAGMENT);
  let param_encoded = utf8_percent_encode(&param, FRAGMENT);

  let data = format!("{}&{}&{}", http_method_encoded, endpoint_encoded, param_encoded);
  let hash = hmacsha1::hmac_sha1(key.as_bytes(), data.as_bytes());
  base64::encode(&hash)
}