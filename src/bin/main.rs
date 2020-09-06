extern crate null;
use null::api;
use null::config::Config;
use std::fs;
use std::fs::File;
use std::io::{Write};

#[tokio::main]
async fn main() {
    let config = Config::new(None).unwrap();
    // fetch_id(&config);
    // bulk_delete(&config);
    // bulk_unretweet(&config);
}

async fn fetch_id(config: &Config) {
    let mut max_id = None;
    let mut file = File::create("id.txt").unwrap();
    loop {
        match api::get_my_tweets(&config, max_id).await {
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

async fn bulk_delete(config: &Config) {
    let ids = fs::read_to_string("id.txt").unwrap();
    let mut ids = ids.split(',').map(|id| id.parse::<u64>().unwrap()).collect::<Vec<_>>();
    ids.sort();
    for n in 0..1000 {
        match api::delete_tweet(&config, ids[n]).await {
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

async fn bulk_unretweet(config: &Config) {
    match api::get_retweeted_tweets(&config, None).await {
        Ok(tweets) => {
            for n in 0..tweets.len() {
                match api::unretweet(&config, tweets[n].id).await {
                    Ok(tweet) => {
                        println!("{} unretweeted.", tweet.text);
                    },
                    Err(e) => {
                        eprintln!("{:#?}", e);
                        break;
                    }
                }
            };
        },
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}