/*
 * @lc app=leetcode.cn id=438 lang=rust
 *
 * [438] 找到字符串中所有字母异位词
 */

// @lc code=start
use std::{collections::HashMap, hash::Hash};

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut need_map: HashMap<char, i32> = HashMap::new();
        let mut window_size = 0;
        let mut wondow_map: HashMap<char, i32> = HashMap::new();
        let p_size = p.len();
        for c in p.chars() {
            *need_map.entry(c).or_insert(0) += 1;
        }
        let mut need_size = need_map.len();
        let (mut left, mut right) = (0, 0);
        let s_chars = s.chars().collect::<Vec<char>>();
        while right < s_chars.len() {
            let c = s_chars[right];
            right += 1;
            if need_map.contains_key(&c) {
                *wondow_map.entry(c).or_insert(0) += 1;
                if wondow_map.get(&c).unwrap() == need_map.get(&c).unwrap() {
                    window_size += 1;
                }
            }
            while right - left >= p_size {
                if window_size == need_size {
                    result.push(left as i32);
                }
                let c = s_chars[left];
                left += 1;
                if need_map.contains_key(&c) {
                    if wondow_map.get(&c).unwrap() == need_map.get(&c).unwrap() {
                        window_size -= 1;
                    }
                    *wondow_map.entry(c).or_insert(0) -= 1;
                }
            }
        }
        result
    }
}
// @lc code=end
