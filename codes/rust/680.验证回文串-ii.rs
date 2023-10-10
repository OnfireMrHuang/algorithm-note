/*
 * @lc app=leetcode.cn id=680 lang=rust
 *
 * [680] 验证回文串 II
 */

// @lc code=start
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let len = s.len();
        let chars: Vec<char> = s.chars().collect();
        let i = 0;
        let j = len - 1;
        let delete_one_val = false;
        return Self::sub_valid_palindrome(&chars, i, j, delete_one_val);
    }

    pub fn sub_valid_palindrome(
        chars: &Vec<char>,
        mut begin: usize,
        mut end: usize,
        mut delete_one_val: bool,
    ) -> bool {
        loop {
            if end <= begin {
                break;
            }
            if chars[begin] != chars[end] {
                if delete_one_val {
                    return false;
                }
                delete_one_val = true;
                let new_end = end - 1;
                if Self::sub_valid_palindrome(chars, begin, new_end, delete_one_val) {
                    return true;
                }
                let new_begin = begin + 1;
                if Self::sub_valid_palindrome(chars, new_begin, end, delete_one_val) {
                    return true;
                }
                return false;
            }
            begin = begin + 1;
            end = end - 1;
        }
        true
    }
}
// @lc code=end
