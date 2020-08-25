use std::env;

use null::api;

fn main() {
    let screen_name = env::var("NAME").expect("Please specify screen_name whose tweets to get.");
    let tweets = api::get_tweets(&screen_name).unwrap();
    println!("{:#?}", tweets);
}
