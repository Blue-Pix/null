use std::env;

use null::api;

fn main() {
    let screen_name = env::var("NAME").expect("Please specify screen_name whose tweets to get.");
    let mut max_id = None;
    for _ in 0..3 {
        let tweets = api::get_tweets(&screen_name, max_id).unwrap();
        if tweets.len() == 0 {
            break;
        }
        println!("{:#?}", tweets);
        println!("=========================================");
        // max_id = tweets.last().map_or(None, |t| Some(t.id-1));
        max_id = Some(tweets.last().unwrap().id-1);
    }
}
