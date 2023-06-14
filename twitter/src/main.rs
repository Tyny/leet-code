struct Twitter {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    fn new() -> Self {
        Twitter {}
    }

    fn post_tweet(&self, user_id: i32, tweet_id: i32) {}

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        vec![]
    }

    fn follow(&self, follower_id: i32, followee_id: i32) {}

    fn unfollow(&self, follower_id: i32, followee_id: i32) {}
}

fn main() {
    let messiId = 1;
    let agueroId = 2;

    let tweetId = 01;

    let obj = Twitter::new();
    obj.post_tweet(messiId, tweetId);
    let ret_2: Vec<i32> = obj.get_news_feed(messiId);
    obj.follow(agueroId, messiId);
    obj.unfollow(agueroId, messiId);
}
