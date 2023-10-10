/*
 * @lc app=leetcode.cn id=150 lang=rust
 *
 * [150] 逆波兰表达式求值
 */

// @lc code=start

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let operator = HashSet::from(["+", "-", "*", "/"]);
        let mut stack: Vec<i32> = Vec::new();
        for token in tokens {
            // 如果是数字则入栈
            if !operator.contains(token.as_str()) {
                stack.push(token.parse::<i32>().unwrap());
                continue;
            }
            let right = stack.pop().unwrap();
            let left = stack.pop().unwrap();
            let result = match token.as_str() {
                "+" => left + right,
                "-" => left - right,
                "*" => left * right,
                "/" => left / right,
                _ => 0,
            };
            stack.push(result);
        }
        stack[0]
    }
}
// @lc code=end
