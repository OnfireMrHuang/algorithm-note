/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let result: Vec<i32> = Vec::new();
        let mut num_map: HashMap<i32, usize> = HashMap::new();
        for i in 0..nums.len() {
            num_map.insert(nums[i], i);
        }
        for i in 0..nums.len() {
            let num = nums[i];
            let diff = target - num;
            if num_map.contains_key(&diff) {
                let diff_index = num_map.get(&diff).unwrap();
                if *diff_index != i {
                    return vec![i as i32, *diff_index as i32];
                }
            }
        }
        result
    }
}
// @lc code=end
