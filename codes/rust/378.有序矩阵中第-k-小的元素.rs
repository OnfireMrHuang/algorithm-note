/*
 * @lc app=leetcode.cn id=378 lang=rust
 *
 * [378] 有序矩阵中第 K 小的元素
 */

// @lc code=start

use std::collections::BinaryHeap;
impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        let row_size = matrix.len();
        for i in 0..row_size {
            let col_size = matrix[0].len();
            let start_idx = i * col_size;
            for j in 0..col_size {
                if start_idx + j < k as usize {
                    heap.push(matrix[i][j]);
                } else {
                    heap.push(matrix[i][j]);
                    heap.pop();
                }
            }
        }
        if heap.len() < 1 {
            return 0;
        }
        heap.peek().unwrap().to_owned()
    }
}
// @lc code=end
