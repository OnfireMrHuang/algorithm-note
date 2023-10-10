/*
 * @lc app=leetcode.cn id=217 lang=rust
 *
 * [217] 存在重复元素
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut cnt_map: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            let cnt = cnt_map.entry(num).or_insert(0);
            *cnt += 1;
            if *cnt > 1 {
                return true;
            }
        }
        false
    }
}
// @lc code=end
