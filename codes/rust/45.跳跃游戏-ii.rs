/*
 * @lc app=leetcode.cn id=45 lang=rust
 *
 * [45] 跳跃游戏 II
 */

// @lc code=start
impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut distance = 0;
        let mut start_steps = 0;
        // 记录跳跃次数
        let mut jump_cnt = 0;
        loop {
            // 如果最后跳跃步数大于等于终点，则一步到位
            if start_steps >= nums.len() - 1 {
                break;
            }
            // 否则选择跳步数跨度最大的一步
            let mut max_steps = 0;
            let mut steps = 0;
            for i in 1..=nums[start_steps] as usize {
                if start_steps + i >= nums.len() - 1 {
                    steps = i;
                    break;
                }
                if i as i32 + nums[start_steps + i] > max_steps {
                    steps = i;
                    max_steps = i as i32 + nums[start_steps + i];
                }
            }
            start_steps = start_steps + steps;
            jump_cnt += 1;
        }
        jump_cnt
    }
}
// @lc code=end
