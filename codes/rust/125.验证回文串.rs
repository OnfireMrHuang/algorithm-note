/*
 * @lc app=leetcode.cn id=125 lang=rust
 *
 * [125] 验证回文串
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let chars = s.chars().collect::<Vec<char>>();
        let (mut left, mut right) = (0, chars.len() - 1);

        while left < right {
            let left_char = chars[left];
            if !left_char.is_ascii_alphanumeric() {
                left += 1;
                continue;
            }
            let right_char = chars[right];
            if !right_char.is_ascii_alphanumeric() {
                right -= 1;
                continue;
            }
            if left_char.to_lowercase().to_string() != right_char.to_lowercase().to_string() {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}
// @lc code=end
