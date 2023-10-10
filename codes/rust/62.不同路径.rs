/*
 * @lc app=leetcode.cn id=62 lang=rust
 *
 * [62] 不同路径
 */

// @lc code=start
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp_above = vec![1; n as usize];
        let mut dp_left = 0;
        let mut dp = 1;
        for _ in 1..m as usize {
            dp_left = 0;
            for j in 0..n as usize {
                dp = dp_above[j] + dp_left;
                dp_left = dp;
                dp_above[j] = dp;
            }
        }
        dp
    }
}
// @lc code=end
