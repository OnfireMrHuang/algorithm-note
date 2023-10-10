/*
 * @lc app=leetcode.cn id=1696 lang=rust
 *
 * [1696] 跳跃游戏 VI
 */

// @lc code=start
use std::collections::VecDeque;

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut dp_stack = VecDeque::new();
        // base case
        dp_stack.push_back((nums[0], 0 as usize));
        for i in 1..nums.len() {
            while i - dp_stack.front().unwrap().1 > k as usize {
                dp_stack.pop_front();
            }
            let ans = dp_stack.front().unwrap().0 + nums[i];
            while !dp_stack.is_empty() && dp_stack.back().unwrap().0 < ans {
                dp_stack.pop_back();
            }
            dp_stack.push_back((ans, i));
        }
        dp_stack.back().unwrap().0
    }
}
// @lc code=end
