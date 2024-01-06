/*
 * @lc app=leetcode.cn id=1044 lang=rust
 *
 * [1044] 最长重复子串
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn longest_dup_substring(s: String) -> String {
        let mut l = 0;
        let mut r = s.len() - 1;
        // 使用二分的方式来查找最长重复子串的长度
        // 如果是重复子串，则往更大的长度查找，否则往更小的长度查找
        while l < r {
            let m = l + ((r - l + 1) >> 1);
            if is_duplicate_present(&s, m) {
                l = m;
            } else {
                r = m - 1;
            }
        }
        find_duplicate(&s, l).unwrap()
    }
}

// 从长度为[0..length]的字符串中找到重复的子串
fn find_duplicate(s: &String, length: usize) -> Option<String> {
    // 采用Rabin-Karp算法来进行查找，而不是暴力查找
    let mut hash: u64 = 0;
    let prime = 29;
    let mut first_entry_power = 1;
    let mut map: HashMap<u64, usize> = HashMap::new();

    let s_arr = s.as_bytes();

    // 首先从[0..length]的字符串中计算出hash值
    for i in 0..length {
        first_entry_power *= prime;
        hash = hash * prime + (s_arr[i] as u64 - 'a' as u64);
    }
    map.insert(hash, 0);
    // 遍历[length..s.len()]，同步计算哈希值，如果哈希值已经存在，则说明有重复的子串
    for i in length..s.len() {
        hash = hash * prime + s_arr[i] as u64 - 'a' as u64;
        hash -= first_entry_power * (s_arr[i - length] as u64 - 'a' as u64);

        if map.contains_key(&hash) {
            let idx = *map.get(&hash).unwrap();
            return Some(s[idx..idx + length].to_string());
        }
        map.insert(hash, i - length + 1);
    }
    None
}

fn is_duplicate_present(s: &String, length: usize) -> bool {
    if length == 0 {
        return true;
    }
    !find_duplicate(s, length).is_none()
}
// @lc code=end
