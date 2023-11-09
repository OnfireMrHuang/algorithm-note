/*
 * @lc app=leetcode.cn id=224 lang=rust
 *
 * [224] 基本计算器
 */

// @lc code=start

use std::vec::Vec;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        // 将字符串转换为字符数组
        let chars = s.chars().collect::<Vec<char>>();
        let ans = Self::helper(&chars, 0);
        ans.1
    }

    fn helper(chars: &Vec<char>, start: usize) -> (usize, i32) {
        let mut digit_stack: Vec<i32> = Vec::new(); // 数字栈
        let mut sign = '+'; // 定义符号
        let mut num = 0; // 定义一个数字
        let mut i = start;
        while i < chars.len() {
            // 获取当前字符
            let c = chars[i];
            i += 1;
            // 如果是数字则缓存数字字符形成完整数字
            if c.is_digit(10 as u32) {
                num = num * 10 + c.to_digit(10 as u32).unwrap() as i32;
                continue;
            }
            // 忽略非法的字符
            if c.is_whitespace() {
                continue;
            }
            match c {
                '(' => {
                    // 当遇到左括号时，递归计算内部括号的表达式并返回末尾索引和结果
                    let (sub_problem_end, sub_problem_val) = Self::helper(chars, i);
                    i = sub_problem_end;
                    match sign {
                        '+' => digit_stack.push(sub_problem_val),
                        '-' => digit_stack.push(-sub_problem_val),
                        _ => {}
                    }
                }
                ')' => {
                    break;
                }
                _ => {}
            }
            match sign {
                '+' => digit_stack.push(num),
                '-' => digit_stack.push(-num),
                '*' => {
                    // 如果是乘法，则弹出数字先进行计算
                    let last_num = digit_stack.pop().unwrap();
                    digit_stack.push(last_num * num);
                }
                '/' => {
                    // 如果是除法，则弹出数字先进行计算
                    let last_num = digit_stack.pop().unwrap();
                    digit_stack.push(last_num / num);
                }
                _ => {}
            }
            // 然后重置数字和符号
            num = 0;
            sign = c;
        }
        // 最后再将缓存的数字压入栈中并对栈中的数字进行累加
        match sign {
            '+' => digit_stack.push(num),
            '-' => digit_stack.push(-num),
            _ => {}
        }
        (i, digit_stack.iter().sum())
    }
}
// @lc code=end
