extern crate null;
use null::config::Config;
use std::fs;
use std::fs::File;
use std::io::{Write};

fn main() {
    let config = Config::new(None).unwrap();
    fetch_id(&config);
    // bulk_delete(&config);
}

#[tokio::main]
async fn fetch_id(config: &Config) {
    let mut max_id = None;
    let mut file = File::create("id.txt").unwrap();
    loop {
        match null::api::get_tweets(&config, max_id).await {
            Ok(tweets) => {
                if tweets.len() == 0 {
                    break;
                }
                let ids = tweets.iter().map(|tweet| tweet.id.to_string()).collect::<Vec<_>>().join(",");
                write!(file, "{},", ids).unwrap();
                max_id = Some(tweets.last().unwrap().id-1);
            },
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        }
    }
}

#[tokio::main]
async fn bulk_delete(config: &Config) {
    let ids = fs::read_to_string("id.txt").unwrap();
    let mut ids = ids.split(',').map(|id| id.parse::<u64>().unwrap()).collect::<Vec<_>>();
    ids.sort();
    for n in 0..1000 {
        match null::api::delete_tweet(&config, ids[n]).await {
            Ok(tweet) => {
                println!("{} deleted.", tweet.id);
            },
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        }
    };
}