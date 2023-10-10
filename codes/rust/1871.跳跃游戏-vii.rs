/*
 * @lc app=leetcode.cn id=1871 lang=rust
 *
 * [1871] 跳跃游戏 VII
 */

// @lc code=start
impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let mut dp = vec![false; s.len()];
        dp[0] = true;
        // 前缀和的目的是求区间内存在值为0并且能到达的下标个数
        let mut pre_sum = vec![0; s.len()];
        for i in 0..min_jump {
            pre_sum[i as usize] = 1;
        }
        let chars = s.chars().into_iter().collect::<Vec<char>>();
        for i in min_jump as usize..chars.len() {
            // 默认沿用之前的前缀和
            pre_sum[i] = pre_sum[i - 1];
            if chars[i] == '0' {
                // 如果i == '0', i可以从[i-max_jump]到[i-min_jump]区间的任何一个值为'0'的下标跳过来
                let mut left_idx = i as i32 - max_jump;
                if left_idx < 0 {
                    left_idx = 0;
                }
                let right_idx = i as i32 - min_jump;
                // first表示left边界的有效值,second表示(left, right]区间的有效值
                let mut first_val = 0;
                let mut second_val = 0;
                if chars[i] == '0' && dp[left_idx as usize] {
                    first_val = 1;
                }
                if right_idx > left_idx {
                    // 我们是需要找(left_idx..right_idx]区间有能到达的0值加left_idx是否为合理值
                    second_val = pre_sum[right_idx as usize] - pre_sum[left_idx as usize];
                }
                if (first_val + second_val) > 0 {
                    dp[i] = true;
                    pre_sum[i] = pre_sum[i - 1] + 1;
                }
            }
        }
        dp[s.len() - 1]
    }
}
// @lc code=end
