/*
 * @lc app=leetcode.cn id=8 lang=rust
 *
 * [8] 字符串转换整数 (atoi)
 */

// @lc code=start

use std::{char, collections::HashMap, hash::Hash};

struct Automaton {
    pub sign: i32,
    pub ans: i64,
    state: String,
    table: HashMap<String, Vec<&'static str>>,
}

impl Automaton {
    // 创建
    fn new() -> Self {
        Self {
            sign: 1,
            ans: 0,
            state: "start".to_string(),
            table: HashMap::from([
                (
                    "start".to_string(),
                    vec!["start", "signed", "in_number", "end"],
                ),
                ("signed".to_string(), vec!["end", "end", "in_number", "end"]),
                (
                    "in_number".to_string(),
                    vec!["end", "end", "in_number", "end"],
                ),
                ("end".to_string(), vec!["end", "end", "end", "end"]),
            ]),
        }
    }

    // 获取字符
    pub fn recv_char(self: &mut Self, c: char) {
        // 转换状态
        self.state = self.table.get(&self.state).unwrap()[Self::get_col(c)].to_string();
        match self.state.as_str() {
            "signed" => {
                if c == '-' {
                    self.sign = -1;
                }
            }
            "in_number" => {
                self.ans = self.ans * 10
                    + (c.to_digit(10).unwrap() as i64 - '0'.to_digit(10).unwrap() as i64);
                if self.sign == 1 {
                    self.ans = self.ans.min(i32::MAX as i64);
                } else {
                    self.ans = self.ans.min(-(i32::MIN as i64));
                }
            }
            _ => {}
        }
    }

    fn get_col(c: char) -> usize {
        if c.is_whitespace() {
            return 0;
        }
        if c == '+' || c == '-' {
            return 1;
        }
        if c.is_numeric() {
            return 2;
        }
        return 3;
    }
}
impl Solution {
    pub fn my_atoi(s: String) -> i64 {
        let mut automaton = Automaton::new();
        for c in s.chars() {
            automaton.recv_char(c);
        }
        automaton.sign as i64 * automaton.ans
    }
}
// @lc code=end
