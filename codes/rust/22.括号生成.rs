/*
 * @lc app=leetcode.cn id=22 lang=rust
 *
 * [22] 括号生成
 */

// @lc code=start
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut left_parenthese_choices = n;
        let mut right_parenthese_choices = n;
        let mut res: Vec<String> = Vec::new();
        Self::backtrace(
            String::new(),
            left_parenthese_choices,
            right_parenthese_choices,
            &mut res,
        );
        res
    }

    fn backtrace(
        mut state: String,
        left_parenthese_choices: i32,
        right_parenthese_choices: i32,
        res: &mut Vec<String>,
    ) {
        // 如果选择完了，该字符串是有效的括号，则添加到状态中
        if left_parenthese_choices == 0 && right_parenthese_choices == 0 {
            if Self::is_valid_parenthese(&state) {
                res.push(state);
            }
            return;
        }
        // 如果左括号还有选择，则可以选择左括号
        if left_parenthese_choices > 0 {
            state.push('(');
            Self::backtrace(
                state.clone(),
                left_parenthese_choices - 1,
                right_parenthese_choices,
                res,
            );
            state.pop();
        }

        if right_parenthese_choices > 0 {
            // 接着选右括号
            state.push(')');
            Self::backtrace(
                state.clone(),
                left_parenthese_choices,
                right_parenthese_choices - 1,
                res,
            );
            state.pop();
        }
    }

    fn is_valid_parenthese(state: &String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for item in state.chars() {
            if item == '(' {
                stack.push(item);
                continue;
            }
            if stack.is_empty() {
                return false;
            }
            if stack.pop().unwrap() != '(' {
                return false;
            }
        }
        if !stack.is_empty() {
            return false;
        }
        true
    }
}
// @lc code=end
