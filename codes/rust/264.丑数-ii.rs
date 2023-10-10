/*
 * @lc app=leetcode.cn id=264 lang=rust
 *
 * [264] 丑数 II
 */

// @lc code=start

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let table = [2, 3, 5];
        let mut ret: Vec<i32> = vec![];
        let mut heap = BinaryHeap::new();
        let mut set = HashSet::new();
        heap.push(Reverse(1));
        for i in 0..n as usize {
            let min = heap.pop().unwrap().0;
            ret.push(min);
            for j in 0..3 {
                let new = min as i64 * table[j];
                if new > i32::MAX as i64 {
                    continue;
                }
                let new = new as i32;
                if set.contains(&new) == false {
                    heap.push(Reverse(new));
                    set.insert(new);
                }
            }
        }
        ret[n as usize - 1]
    }
}
// @lc code=end
