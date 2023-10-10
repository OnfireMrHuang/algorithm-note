/*
 * @lc app=leetcode.cn id=887 lang=rust
 *
 * [887] 鸡蛋掉落
 */

// @lc code=start
impl Solution {
    pub fn super_egg_drop(k: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize + 1]; k as usize + 1];
        // base case: m=0,肯定n就等于0或者k等于0，n=0
        // m最多可以测试n次, i可以测试1到k枚鸡蛋
        let mut m = 0;
        while dp[k as usize][m] < n {
            m += 1;
            for i in 1..=k as usize {
                dp[i][m] = dp[i - 1][m - 1] + dp[i][m - 1] + 1;
            }
        }
        m as i32
    }
}
// @lc code=end
