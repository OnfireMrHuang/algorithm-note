/*
 * @lc app=leetcode.cn id=560 lang=rust
 *
 * [560] 和为 K 的子数组
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut res = 0;
        let mut pre_sum = 0;
        let mut pre_sum_map: HashMap<i32, usize> = HashMap::new();
        pre_sum_map.insert(0, 1); // 默认空序列的和为0
        for i in 0..nums.len() {
            let cur_sum = pre_sum + nums[i];
            let last_sum = cur_sum - k;
            if pre_sum_map.contains_key(&last_sum) {
                res += pre_sum_map.get(&last_sum).unwrap();
            }
            let sum_val = pre_sum_map.entry(cur_sum).or_insert(0);
            *sum_val += 1;
            pre_sum = cur_sum;
        }
        res as i32
    }
}
// @lc code=end
