/*
 * @lc app=leetcode.cn id=55 lang=rust
 *
 * [55] 跳跃游戏
 */

// @lc code=start
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.is_empty() {
            return true;
        }
        let size = nums.len() - 1;
        let mut start_steps = 0;
        let mut res = true;
        loop {
            // 最大的选择步数
            let max_select_steps = nums[start_steps];
            if start_steps + max_select_steps as usize >= size {
                res = true;
                break;
            }
            let mut steps = 0;
            let mut max_steps = 0;
            // 选择跨度最大的一步
            for i in 0..=max_select_steps {
                if i + nums[start_steps + i as usize] > max_steps {
                    max_steps = i + nums[start_steps + i as usize];
                    steps = i;
                }
            }
            // 说明原地踏步，永远都到不了
            if steps == 0 {
                res = false;
                break;
            }
            start_steps += steps as usize;
        }
        res
    }
}
// @lc code=end
