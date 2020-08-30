extern crate null;
use null::config::Config;

fn main() {
    let config = Config::new(None).unwrap();
    let mut max_id = None;
    for _ in 0..3 {
        let tweets = null::api::get_tweets(&config, max_id).unwrap();
        if tweets.len() == 0 {
            break;
        }
        println!("{:#?}", tweets);
        println!("=========================================");
        // max_id = tweets.last().map_or(None, |t| Some(t.id-1));
        max_id = Some(tweets.last().unwrap().id-1);
    }
}
