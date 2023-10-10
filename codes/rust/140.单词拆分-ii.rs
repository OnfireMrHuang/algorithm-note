/*
 * @lc app=leetcode.cn id=140 lang=rust
 *
 * [140] 单词拆分 II
 */

// @lc code=start
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut set = HashSet::new();
        // 首先将所有单词放入到集合中
        for word in word_dict {
            set.insert(word);
        }
        let size = s.len();
        let mut ans: Vec<String> = Vec::new();
        let mut path = String::new();
        Self::backtrack(&s, 0, size, &set, &mut path, &mut ans);
        ans
    }

    fn backtrack(
        s: &str,
        start_idx: usize,
        size: usize,
        set: &HashSet<String>,
        path: &mut String,
        ans: &mut Vec<String>,
    ) {
        // 回退点, 说明超过字符串索引了，整个字符串能被字典中的单词拼装
        if start_idx == size {
            ans.push(path[..path.len() - 1].to_string());
        }
        for i in start_idx..size {
            // 如果拼接的单词不在字典中，则继续往后走
            if !set.contains(&s[start_idx..=i].to_string()) {
                continue;
            }
            // 如果在，则继续找[i+1..size]的字符
            path.push_str(&s[start_idx..=i]);
            path.push(' ');
            Self::backtrack(s, i + 1, size, set, path, ans);
            // 还原路径
            path.pop();
            for _ in start_idx..=i {
                path.pop();
            }
        }
    }
}
// @lc code=end
