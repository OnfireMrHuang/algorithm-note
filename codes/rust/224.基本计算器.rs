/*
 * @lc app=leetcode.cn id=224 lang=rust
 *
 * [224] 基本计算器
 */

// @lc code=start

use std::vec::Vec;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut digit_stack: Vec<i32> = Vec::new(); // 数字栈
        let mut ops_stack: Vec<char> = Vec::new(); // 操作符栈

        let char_vec = s.chars();
        let mut digit: i32 = -1;

        // 初始化，往数字栈默认添加一个0值(兼容-1场景)
        digit_stack.push(0);

        let mut left_parentheses_flag = false; //左括号标记
        for c in char_vec {
            match c {
                '+' | '-' => {
                    // 首先判断是否有数字，有则先入数字栈
                    if digit != -1 {
                        digit_stack.push(digit);
                    }
                    // 如果上一个元素是左括号，则默认压入数字0
                    if left_parentheses_flag {
                        digit_stack.push(0);
                    }
                    // 将栈里的元素做一个计算
                    let pop_op = Self::calc(&mut digit_stack, &mut ops_stack);
                    if pop_op == Some('(') {
                        ops_stack.push('(');
                    }
                    // 将数字重置为-1
                    digit = -1;
                    // 将操作符压栈
                    ops_stack.push(c);
                    left_parentheses_flag = false;
                }
                '(' => {
                    // 首先将数字入栈
                    if digit != -1 {
                        digit_stack.push(digit);
                    }
                    // 重置数字
                    digit = -1;
                    // 入栈
                    ops_stack.push(c);
                    left_parentheses_flag = true;
                }
                ')' => {
                    if digit != -1 {
                        digit_stack.push(digit);
                    }
                    // 重置数字
                    digit = -1;
                    while Self::calc(&mut digit_stack, &mut ops_stack) != Some('(') {}
                    left_parentheses_flag = false;
                }
                '0'..='9' => {
                    let num = c.to_digit(10).unwrap() as i32;
                    if digit == -1 {
                        digit = num;
                    } else {
                        digit *= 10;
                        digit += num;
                    }
                    left_parentheses_flag = false;
                }
                _ => {}
            }
        }
        // 如果还有缓存的数字则压栈
        if digit != -1 {
            digit_stack.push(digit);
        }
        // 如果栈里面还有数据，则继续弹出
        while Self::calc(&mut digit_stack, &mut ops_stack) != None {}
        digit_stack.pop().unwrap()
    }

    // 执行计算并返回操作符
    fn calc(digit_stack: &mut Vec<i32>, ops_stack: &mut Vec<char>) -> Option<char> {
        let op_value = ops_stack.pop();
        if op_value == None {
            return None;
        }
        let op = op_value.unwrap();
        // 从操作符栈里pop出一个操作符号
        match op {
            '+' => {
                let right_digit = digit_stack.pop().unwrap();
                let left_digit = digit_stack.pop().unwrap();
                let new_digit = right_digit + left_digit;
                digit_stack.push(new_digit);
                Some(op)
            }
            '-' => {
                let right_digit = digit_stack.pop().unwrap();
                let left_digit = digit_stack.pop().unwrap();
                let new_digit = left_digit - right_digit;
                digit_stack.push(new_digit);
                Some(op)
            }
            '(' => Some(op),
            _ => None,
        }
    }
}
// @lc code=end
