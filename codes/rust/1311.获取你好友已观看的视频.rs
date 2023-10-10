/*
 * @lc app=leetcode.cn id=1311 lang=rust
 *
 * [1311] 获取你好友已观看的视频
 */

// @lc code=start
impl Solution {
    pub fn watched_videos_by_friends(
        watched_videos: Vec<Vec<String>>,
        friends: Vec<Vec<i32>>,
        id: i32,
        k: i32,
    ) -> Vec<String> {
        let mut visited = vec![false; friends.len()];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((id as usize, 0)); // (id, level)
        visited[id as usize] = true;
        let mut friend_k: Vec<i32> = vec![];
        while !queue.is_empty() {
            let (id, level) = queue.pop_front().unwrap();
            if level == k {
                friend_k.push(id as i32);
                continue;
            }
            for friend in friends[id].iter() {
                if !visited[*friend as usize] {
                    visited[*friend as usize] = true;
                    queue.push_back((*friend as usize, level + 1));
                }
            }
        }
        let mut count = std::collections::HashMap::new();
        for friend in friend_k.iter() {
            for video in watched_videos[*friend as usize].iter() {
                *count.entry(video).or_insert(0) += 1;
            }
        }
        let mut count_vec: Vec<(&String, &i32)> =
            count.iter().map(|(video, count)| (*video, count)).collect();
        count_vec.sort_by(|a, b| {
            if a.1 == b.1 {
                return a.0.cmp(b.0);
            }
            a.1.cmp(b.1)
        });
        count_vec.iter().map(|x| x.0.to_string()).collect()
    }
}
// @lc code=end
