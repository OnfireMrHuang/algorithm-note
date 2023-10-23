/*
 * @lc app=leetcode.cn id=41 lang=rust
 *
 * [41] 缺失的第一个正数
 */

// @lc code=start
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let size = nums.len();
        for i in 0..size {
            // 如果nums[i]在[1, size]的范围内, 并且nums[i] != nums[nums[i] - 1], 则一直交换，直到符合条件
            // 其中while + (nums[i] != nums[nums[i] - 1])的意思是一直循环到将下表为nums[i]-1的元素放到正确的位置
            while nums[i] > 0 && nums[i] <= size as i32 && nums[i] != nums[nums[i] as usize - 1] {
                let index = nums[i] as usize - 1;
                nums.swap(i, index);
            }
        }
        // 遍历数组，如果nums[i] != i + 1, 则返回i + 1
        for i in 0..size {
            if nums[i] != i as i32 + 1 {
                return i as i32 + 1;
            }
        }
        return size as i32 + 1;
    }
}
// @lc code=end
