/*
 * @lc app=leetcode.cn id=131 lang=rust
 *
 * [131] 分割回文串
 */

// @lc code=start
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::new();
        if s.is_empty() {
            return result;
        }
        let mut path: Vec<String> = Vec::new();
        Self::traver(&s[0..], &mut path, &mut result);
        result
    }

    fn traver(sub_str: &str, path: &mut Vec<String>, result: &mut Vec<Vec<String>>) {
        // 首先判断自己是不是回文串，如果是直接追加到结果
        if sub_str.is_empty() {
            result.push(path.clone());
            return;
        }
        // 如果不是则开始分割尝试
        for i in 0..sub_str.len() {
            // 切分成两段: 一段从0..i+1, 一段从i+1..sub_str.len()
            let curr = &sub_str[0..i + 1];
            if !Self::is_palindrome(curr) {
                continue;
            }
            path.push(curr.to_string());
            let next = &sub_str[i + 1..];
            Self::traver(next, path, result);
            path.pop();
        }
    }

    pub fn is_palindrome(s: &str) -> bool {
        // 空串等于回文字符串
        if s.is_empty() {
            return true;
        }
        // 字符本身也是回文串
        if s.len() == 1 {
            return true;
        }
        let (mut left, mut right) = (0, s.len() - 1);
        while left < right {
            let left_char = &s[left..left + 1];
            let right_char = &s[right..right + 1];
            if left_char.to_lowercase() != right_char.to_lowercase() {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}
// @lc code=end
