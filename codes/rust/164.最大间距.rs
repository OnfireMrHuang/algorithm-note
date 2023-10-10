/*
 * @lc app=leetcode.cn id=164 lang=rust
 *
 * [164] 最大间距
 */

// @lc code=start
impl Solution {
    pub fn maximum_gap(input: Vec<i32>) -> i32 {
        let mut nums = input; // 获取所有权并设置为可改写
        let length = nums.len();
        if length < 2 {
            return 0;
        }
        let mut max_value = 0;
        // 遍历数组拿到最大值 - O(n)
        for i in 0..length {
            if nums[i] > max_value {
                max_value = nums[i];
            }
        }
        let mut exp = 1;
        // 从个、十、百、千位逐位进行排序
        while max_value >= exp {
            let mut dight_buckets = vec![0; 10];
            let mut temp_nums = vec![0; length];
            // 计算数字出现的次数
            for i in 0..length {
                let dight = (nums[i] / exp % 10) as usize;
                dight_buckets[dight] += 1;
            }
            // 计算数字前面有多少位
            for i in 1..10 {
                dight_buckets[i] += dight_buckets[i - 1];
            }
            for i in (0..length).rev() {
                let dight = (nums[i] / exp % 10) as usize;
                temp_nums[dight_buckets[dight] - 1] = nums[i];
                dight_buckets[dight] -= 1;
            }
            // 排序完之后再覆盖原数组，在改次排序的基础上进一位后再排序
            nums.copy_from_slice(&temp_nums);
            exp *= 10;
        }
        // 最后再从已经排序好的数组中找出间隔最大值
        let mut max_interval = 0;
        for i in 1..length {
            if nums[i] - nums[i - 1] > max_interval {
                max_interval = nums[i] - nums[i - 1];
            }
        }
        max_interval
    }
}
// @lc code=end
