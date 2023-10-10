/*
 * @lc app=leetcode.cn id=434 lang=rust
 *
 * [434] 字符串中的单词数
 */

// @lc code=start
impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let mut count = 0;
        let mut flag = false;
        for c in s.chars() {
            if c == ' ' {
                if flag {
                    count += 1;
                    flag = false;
                }
            } else {
                flag = true;
            }
        }
        if flag {
            count += 1;
        }
        count
    }
}
// @lc code=end
