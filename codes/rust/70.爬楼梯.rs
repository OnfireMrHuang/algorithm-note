/*
 * @lc app=leetcode.cn id=70 lang=rust
 *
 * [70] 爬楼梯
 */

// @lc code=start

use std::{collections::HashMap, hash::Hash};

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 1 {
            return 0;
        }
        // 初始化一个数组，下标为数字，值为爬楼梯的方法数
        let mut dp = vec![0; n as usize + 3];
        dp[1] = 1;
        dp[2] = 2;
        for i in 3..=n as usize {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n as usize]
    }
}
// @lc code=end
