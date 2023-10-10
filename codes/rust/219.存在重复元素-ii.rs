/*
 * @lc app=leetcode.cn id=219 lang=rust
 *
 * [219] 存在重复元素 II
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        // 其实哈希表我只需要记录最大的一个索引就好了
        let mut idx_map: HashMap<i32, i32> = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            if let Some(max_idx) = idx_map.get(num) {
                if i as i32 - max_idx <= k {
                    return true;
                }
            }
            idx_map.insert(num.to_owned(), i as i32);
        }
        false
    }
}
// @lc code=end
