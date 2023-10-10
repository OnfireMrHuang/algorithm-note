/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut max_substring_len = 0;
        let mut max_substring_start = 0;
        let mut max_substring_end = 0;
        for i in 0..s.len() {
            for j in i..s.len() {
                let sub_str: Vec<u8> = s[i..=j].into();
                let rev_sub_str: Vec<u8> = s[i..=j].bytes().rev().collect();
                if sub_str == rev_sub_str {
                    if max_substring_len < j - i + 1 {
                        max_substring_len = j - i + 1;
                        max_substring_start = i;
                        max_substring_end = j;
                    }
                }
            }
        }
        s[max_substring_start..=max_substring_end].to_string()
    }
}
// @lc code=end
