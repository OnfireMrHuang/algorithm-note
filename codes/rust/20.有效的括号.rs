/*
* @lc app=leetcode.cn id=20 lang=rust
*
* [20] 有效的括号
*
* 给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。
*/

// @lc code=start

use std::{collections::HashMap, hash::Hash};

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        let mut parentheses_map: HashMap<char, char> = HashMap::new();
        parentheses_map.insert(')', '(');
        parentheses_map.insert(']', '[');
        parentheses_map.insert('}', '{');
        for c in s.chars() {
            if c == '(' || c == '[' || c == '{' {
                stack.push(c);
                continue;
            }
            if c == ')' || c == ']' || c == '}' {
                // 如果此时stack为空，则说明没有左括号，直接返回false
                if stack.is_empty() {
                    return false;
                }
                let top = stack.pop().unwrap();
                if parentheses_map.get(&c) == Some(&top) {
                    continue;
                } else {
                    return false;
                }
            }
        }
        if !stack.is_empty() {
            return false;
        }
        true
    }
}
// @lc code=end
