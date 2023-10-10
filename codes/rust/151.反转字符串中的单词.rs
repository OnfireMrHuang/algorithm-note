/*
 * @lc app=leetcode.cn id=151 lang=rust
 *
 * [151] 反转字符串中的单词
 */

// @lc code=start
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut array: Vec<String> = Vec::new();
        let mut word = String::new();
        for c in s.chars() {
            if c.is_whitespace() {
                if !word.is_empty() {
                    array.push(word);
                    word = String::new();
                }
            } else {
                word.push(c);
            }
        }
        if !word.is_empty() {
            array.push(word);
        }
        let mut result = String::new();
        for w in array.iter().rev() {
            result.push_str(w);
            result.push(' ');
        }
        result.pop();
        result
    }
}
// @lc code=end
