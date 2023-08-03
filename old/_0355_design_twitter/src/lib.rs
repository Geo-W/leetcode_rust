use std::collections::{HashMap, HashSet};

/// Design a simplified version of Twitter where users can post tweets, follow/unfollow another user, and is able to see the 10 most recent tweets in the user's news feed.
struct Twitter {
    posts: HashMap<i32, Vec<(u32, i32)>>,
    following: HashMap<i32, HashSet<i32>>,
    seq: u32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    fn new() -> Self {
        Twitter { posts: HashMap::new(), following: HashMap::new(), seq: 0 }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        if self.posts.contains_key(&user_id) {
            self.posts.get_mut(&user_id).unwrap().push((self.seq, tweet_id));
        } else {
            self.posts.insert(user_id, vec![(self.seq, tweet_id)]);
        }
        self.seq += 1;
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        match self.following.get(&user_id) {
            None => {
                self.posts.get(&user_id).unwrap_or(&Vec::<(u32, i32)>::new())
                    .iter()
                    .rev()
                    .take(10)
                    .map(|(_u, i)| *i)
                    .collect()
            }
            Some(v) => {
                let mut ret: Vec<(u32, i32)> = self.posts.get(&user_id).unwrap_or(&Vec::<(u32,i32)>::new()).clone();
                v.iter().for_each(|i| {
                    ret.extend(self.posts.get(i).unwrap_or(&Vec::<(u32, i32)>::new()));
                });
                ret.sort_by_key(|k| k.0);
                ret.iter().rev().take(10).map(|(_u, i)| *i).collect()
            }
        }
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        match self.following.get_mut(&follower_id) {
            None => {
                self.following.insert(follower_id, HashSet::from([followee_id]));
            }
            Some(v) => {
                v.insert(followee_id);
            }
        }
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(v) = self.following.get_mut(&follower_id) {
            v.remove(&followee_id);
        }
    }
}

/**
 * Your Twitter object will be instantiated and called as such:
 * let obj = Twitter::new();
 * obj.post_tweet(userId, tweetId);
 * let ret_2: Vec<i32> = obj.get_news_feed(userId);
 * obj.follow(followerId, followeeId);
 * obj.unfollow(followerId, followeeId);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut obj = Twitter::new();
        for i in [[1,5],[1,3],[1,101],[1,13],[1,10],[1,2],[1,94],[1,505],[1,333],[1,22],[1,11]]{
            obj.post_tweet(i[0], i[1]);
        }
        let ret_2: Vec<i32> = obj.get_news_feed(1);
        assert_eq!(ret_2, vec![11,22,333,505,94,2,10,13,101,3]);
    }
}
