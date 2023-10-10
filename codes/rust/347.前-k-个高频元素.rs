/*
 * @lc app=leetcode.cn id=347 lang=rust
 *
 * [347] 前 K 个高频元素
 */

// @lc code=start
use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
#[derive(PartialEq, Eq)]
struct FreVal {
    fre_count: i32,
    val: i32,
}

impl Ord for FreVal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.fre_count.cmp(&other.fre_count)
    }
}

impl PartialOrd for FreVal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let num_fre_map = Self::build_num_frequency_map(nums);
        let mut ret = vec![];
        let mut heap = BinaryHeap::new();
        let mut index = 0;
        for item in num_fre_map.iter() {
            if index < k {
                heap.push(Reverse(FreVal {
                    fre_count: item.1.to_owned(),
                    val: item.0.to_owned(),
                }));
                index += 1;
                continue;
            }
            let fre_val = heap.peek().unwrap();
            if item.1.to_owned() < fre_val.0.fre_count {
                continue;
            }
            let _ = heap.pop();
            heap.push(Reverse(FreVal {
                fre_count: item.1.to_owned(),
                val: item.0.to_owned(),
            }))
        }
        for item in heap.into_iter() {
            ret.push(item.0.val);
        }
        ret.sort();
        ret
    }
    pub fn build_num_frequency_map(nums: Vec<i32>) -> Box<HashMap<i32, i32>> {
        let mut hash_map = Box::new(HashMap::new());
        for num in nums {
            let fre = hash_map.get_mut(&num);
            if fre.is_none() {
                hash_map.insert(num, 1);
                continue;
            }
            let val = fre.unwrap();
            *val += 1;
        }
        hash_map
    }
}
// @lc code=end
