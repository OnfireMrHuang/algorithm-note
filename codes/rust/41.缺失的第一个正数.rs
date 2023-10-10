/*
 * @lc app=leetcode.cn id=41 lang=rust
 *
 * [41] 缺失的第一个正数
 */

// @lc code=start
impl Solution {
    pub fn first_missing_positive(param: Vec<i32>) -> i32 {
        let mut nums = param;
        let length = nums.len();
        for i in 0..length {
            while nums[i] > 0 && nums[i] <= length as i32 && nums[i] != nums[nums[i] as usize - 1] {
                let index = nums[i];
                nums.swap(i, index as usize - 1);
            }
        }
        for i in 0..length {
            if nums[i] != i as i32 + 1 {
                return i as i32 + 1;
            }
        }
        return length as i32 + 1;
    }
}
// @lc code=end
