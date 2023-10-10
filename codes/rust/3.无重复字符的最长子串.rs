/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_range_length = 0;
        let mut occ = HashMap::new(); // 定义一个哈希表，用来保存出现过的字符
        let mut rk = 0; // 定义一个指向右窗口的下标索引
        for i in 0..s.len() {
            if i != 0 {
                occ.remove(&s[i - 1..i]);
            }
            // 设置窗口，小于数组并且没有出现重复值
            while rk < s.len() && occ.get(&s[rk..rk + 1]) == None {
                occ.insert(&s[rk..rk + 1], true);
                rk += 1;
            }
            if rk - i > max_range_length {
                max_range_length = rk - i;
            }
        }
        max_range_length as i32
    }
}
// @lc code=end
