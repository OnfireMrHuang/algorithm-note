/*
 * @lc app=leetcode.cn id=355 lang=rust
 *
 * [355] 设计推特
 */

// @lc code=start
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    vec,
};
struct Twitter {
    timestamp: i32,                           // 推文全局逻辑时间戳
    follow_map: HashMap<i32, HashSet<i32>>,   // 记录一个用户关注了哪些用户,默认关注着是自己
    tweet_map: HashMap<i32, Vec<(i32, i32)>>, // 记录用户自己的推文，每个推文附属发布逻辑时间戳
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {
    fn new() -> Self {
        Twitter {
            timestamp: 0,
            follow_map: HashMap::new(),
            tweet_map: HashMap::new(),
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        // 首先检查用户的关注列表是否存在
        if self.follow_map.get(&user_id).is_none() {
            self.follow_map.insert(user_id, HashSet::from([user_id]));
        }
        // 再检查用户的推文列表是否存在
        if self.tweet_map.get(&user_id).is_none() {
            self.tweet_map.insert(user_id, vec![]);
        }
        // 写入该用户的推文
        let tweet_list = self.tweet_map.get_mut(&user_id).unwrap();
        tweet_list.push((tweet_id, self.timestamp));
        // 逻辑时间戳累积1
        self.timestamp += 1;
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        // 首先检查follower_id的关注列表是否存在
        if self.follow_map.get(&follower_id).is_none() {
            self.follow_map
                .insert(follower_id, HashSet::from([follower_id]));
        }
        // 再检查followee_id的关注列表是否存在
        if self.follow_map.get(&followee_id).is_none() {
            self.follow_map
                .insert(followee_id, HashSet::from([followee_id]));
        }
        // 将被关注者添加到关注者的集合中
        let follow_set = self.follow_map.get_mut(&follower_id).unwrap();
        follow_set.insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        // 检查followee_id的关注列表是否存在
        if self.follow_map.get(&follower_id).is_none() {
            self.follow_map
                .insert(follower_id, HashSet::from([follower_id]));
        }
        // 将被关注者从关注者的集合中删除
        let follow_set = self.follow_map.get_mut(&follower_id).unwrap();
        follow_set.remove(&followee_id);
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        // 检查该用户是否存在
        if self.follow_map.get(&user_id).is_none() {
            return vec![];
        }
        // 获取该用户的关注列表
        let follow_set = self.follow_map.get(&user_id).unwrap();
        // 创建优先级队列(默认大顶堆)，保存(时间戳,推文)的元素，统计时间戳最近的10个
        let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        for follow_user in follow_set {
            if self.tweet_map.get(follow_user).is_none() {
                continue;
            }
            // 获取关注的用户，查看关注用户的最近10条推文,并插入优先级队列
            let twitter_list = self.tweet_map.get(follow_user).unwrap().iter().take(11);
            for (twitter, ts) in twitter_list {
                heap.push((ts.to_owned(), twitter.to_owned()));
            }
        }
        // 从优先级队列中取出最近的10条
        let mut ans = vec![];
        while !heap.is_empty() {
            if ans.len() >= 10 {
                break;
            }
            let item = heap.pop().unwrap();
            ans.push(item.1);
        }
        ans
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
// @lc code=end

