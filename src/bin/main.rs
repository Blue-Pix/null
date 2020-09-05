extern crate null;
use null::config::Config;

#[tokio::main]
async fn main() {
    let config = Config::new(None).unwrap();
    
    // let mut max_id = None;
    // for _ in 0..3 {
    //     let tweets = null::api::get_tweets(&config, max_id).unwrap();
    //     if tweets.len() == 0 {
    //         break;
    //     }
    //     println!("{:#?}", tweets);
    //     println!("=========================================");
    //     // max_id = tweets.last().map_or(None, |t| Some(t.id-1));
    //     max_id = Some(tweets.last().unwrap().id-1);
    // }
    
    let tweets = null::api::get_tweets(&config, None).await.unwrap();
    let id = tweets.first().unwrap().id;
    match null::api::delete_tweet(&config, id).await {
        Ok(t) => println!("Following tweet is deleted: {}", t.text),
        Err(e) => eprintln!("Failed to delete tweet: {}", e),
    }

    // let token = null::oauth::get_request_token();
    // println!("{:?}", token);
}
