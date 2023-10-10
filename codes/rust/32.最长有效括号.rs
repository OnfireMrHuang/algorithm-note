/*
 * @lc app=leetcode.cn id=32 lang=rust
 *
 * [32] 最长有效括号
 */

// @lc code=start
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut max_valid_len = 0;
        let mut stack: Vec<i32> = Vec::new();
        let char_arr = s.chars().collect::<Vec<char>>();
        stack.push(-1);
        for i in 0..char_arr.len() {
            if char_arr[i] == '(' {
                stack.push(i as i32);
                continue;
            }
            if stack.is_empty() {
                stack.push(i as i32);
                continue;
            }
            let index = stack.pop().unwrap();
            if index == -1 || char_arr[index as usize] == ')' {
                stack.push(i as i32);
                continue;
            }
            if let Some(start_index) = stack.last() {
                if i as i32 - start_index > max_valid_len {
                    max_valid_len = i as i32 - start_index;
                }
            }
        }
        max_valid_len
    }
}
// @lc code=end
