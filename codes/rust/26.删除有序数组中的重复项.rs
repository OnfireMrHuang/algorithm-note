/*
 * @lc app=leetcode.cn id=26 lang=rust
 *
 * [26] 删除有序数组中的重复项
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut res = 1;
        // 设置一个临时值
        let mut temp_val = nums[0];
        for i in 1..nums.len() {
            // 如果当前值大于上一个值，则更新临时值继续遍历
            if nums[i] <= temp_val {
                // 否则就是找到重复值或者废弃值,然后找到下一个比临时值大的数，交换
                let mut j = i;
                while j < nums.len() - 1 && nums[j] <= temp_val {
                    j += 1;
                }
                // 找到最后都没有比temp_val数值大的就推出
                if nums[j] <= temp_val {
                    break;
                }
                nums[i] = nums[j];
                nums[j] = temp_val;
            }
            res += 1;
            temp_val = nums[i];
        }
        res
    }
}
// @lc code=end
