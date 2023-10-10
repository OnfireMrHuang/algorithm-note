/*
 * @lc app=leetcode.cn id=496 lang=rust
 *
 * [496] 下一个更大元素 I
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = vec![-1; nums1.len()];
        let mut map = HashMap::new();
        // 首先对nums1进行哈希映射
        for i in 0..nums1.len() {
            map.insert(nums1[i], i);
        }
        // 然后对nums2进行单调栈遍历
        // 按顺序入栈，如果找到一个比栈顶元素大的并且栈顶元素在nums1中存在，那么当前要入栈的这个元素就是
        // 栈顶元素的下一个大的值
        let mut stack: Vec<usize> = Vec::new();
        for (idx, value) in nums2.iter().enumerate() {
            while !stack.is_empty() && value.to_owned() > nums2[stack.last().unwrap().to_owned()] {
                let pos = nums2[stack.pop().unwrap()];
                if let Some(num1_idx) = map.get(&pos) {
                    result[num1_idx.to_owned()] = value.to_owned();
                }
            }
            stack.push(idx);
        }
        result
    }
}
// @lc code=end
