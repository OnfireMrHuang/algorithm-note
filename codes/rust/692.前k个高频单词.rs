/*
 * @lc app=leetcode.cn id=692 lang=rust
 *
 * [692] 前K个高频单词
 */

// @lc code=start
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut map = HashMap::new();
        let mut heap = BinaryHeap::new();
        // 创建映射
        for word in words {
            *map.entry(word).or_insert(0) += 1;
        }
        // 建立堆
        for (key, value) in map.iter() {
            heap.push((Reverse(value), key));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        // 最后再遍历堆得到
        let mut result = Vec::new();
        while !heap.is_empty() {
            let item = heap.pop().unwrap().1.to_string();
            result.push(item);
        }
        result.reverse();
        result
    }
}
// @lc code=end
