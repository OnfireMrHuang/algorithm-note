/*
 * @lc app=leetcode.cn id=75 lang=rust
 *
 * [75] 颜色分类
 */

// @lc code=start
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut p0 = 0;
        let mut p1 = 0;
        for i in 0..nums.len() {
            if nums[i] == 1 {
                let temp = nums[i];
                nums[i] = nums[p1];
                nums[p1] = temp;
                p1 += 1;
            } else if nums[i] == 0 {
                let temp = nums[i];
                nums[i] = nums[p0];
                nums[p0] = temp;
                if p0 < p1 {
                    let temp = nums[i];
                    nums[i] = nums[p1];
                    nums[p1] = temp;
                }
                p0 += 1;
                p1 += 1;
            }
        }
    }
}
// @lc code=end
