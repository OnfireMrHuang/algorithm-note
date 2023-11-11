/*
 * @lc app=leetcode.cn id=496 lang=rust
 *
 * [496] 下一个更大元素 I
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![-1; nums1.len()];

        // 定一个单调栈和一个哈希表，分别用来存储从栈顶到栈底单调递增的元素和数值的下一个更大元素的映射值
        let mut stack: Vec<i32> = Vec::new();
        let mut table: HashMap<i32, i32> = HashMap::new();
        // 逆序遍历nums2,求出nums2中每个元素的下一个更大元素
        for i in (0..nums2.len()).rev() {
            // 如果栈顶元素小于当前元素，就踢掉栈顶元素，知道找到下一个更大的元素或栈底
            while !stack.is_empty() && stack.last().unwrap() < &nums2[i] {
                stack.pop();
            }
            // 如果存在下一个更大元素，就将其存入哈希表
            if !stack.is_empty() {
                table.insert(nums2[i], stack.last().unwrap().to_owned());
            }
            // 将当前元素入栈
            stack.push(nums2[i]);
        }

        // 接着遍历nums1, 从哈希表中找到每个元素的下一个更大元素
        for i in 0..nums1.len() {
            if table.contains_key(&nums1[i]) {
                ans[i] = table.get(&nums1[i]).unwrap().to_owned();
            }
        }
        ans
    }
}
// @lc code=end
