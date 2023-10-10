/*
 * @lc app=leetcode.cn id=227 lang=rust
 *
 * [227] 基本计算器 II
 */

// @lc code=start
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        let mut sign = '+';
        let mut num = 0;
        for (i, c) in s.chars().enumerate() {
            if c.is_digit(10) {
                num = num * 10 + (c.to_digit(10).unwrap() - '0'.to_digit(10).unwrap()) as i32;
            }
            if (!c.is_digit(10) && c != ' ') || i == s.len() - 1 {
                match sign {
                    '+' => {
                        stack.push(num);
                    }
                    '-' => stack.push(-1 * num),
                    '*' => {
                        let tmp = stack.pop().unwrap();
                        stack.push(tmp * num);
                    }
                    '/' => {
                        let tmp = stack.pop().unwrap();
                        stack.push(tmp / num);
                    }
                    _ => {}
                }
                sign = c;
                num = 0;
            }
        }
        if num != 0 {
            stack.push(num);
        }
        let mut sum = 0;
        while !stack.is_empty() {
            sum += stack.pop().unwrap();
        }
        sum
    }
}
// @lc code=end
