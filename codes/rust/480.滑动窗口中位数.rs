/*
 * @lc app=leetcode.cn id=480 lang=rust
 *
 * [480] 滑动窗口中位数
 */

// @lc code=start

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct DualHeap {
    // 大根堆，维护较小的一半元素
    small: BinaryHeap<i32>,
    // 小根堆，维护较大的一半元素
    large: BinaryHeap<Reverse<i32>>,
    // 哈希表，记录延迟删除的元素,key为元素，value为需要删除的次数
    delayed: HashMap<i32, i32>,
    // small和large当前包含的元素个数，需要扣除被【延迟删除】的元素
    small_size: usize,
    large_size: usize,
    // 元素个数大小
    k: usize,
}

impl DualHeap {
    fn new(k: usize) -> DualHeap {
        DualHeap {
            small: BinaryHeap::new(),
            large: BinaryHeap::new(),
            delayed: HashMap::new(),
            small_size: 0,
            large_size: 0,
            k,
        }
    }

    // 获取中位数
    fn get_median(&self) -> f64 {
        if self.k & 1 == 1 {
            return self.small.peek().unwrap().to_owned() as f64;
        }
        let left_val = self.small.peek().unwrap().to_owned() as f64;
        let right_val = self.large.peek().unwrap().0.to_owned() as f64;
        (left_val + right_val) / 2.0
    }

    // 插入数据
    fn insert(&mut self, num: i32) {
        if self.small.is_empty() || num <= self.small.peek().unwrap().to_owned() {
            self.small.push(num);
            self.small_size += 1;
        } else {
            self.large.push(Reverse(num));
            self.large_size += 1;
        }
        self.make_balance()
    }

    // 擦除数据
    fn erase(&mut self, num: i32) {
        let mut count = 0;
        if let Some(cnt) = self.delayed.get(&num) {
            count = cnt.to_owned();
        }
        count += 1;
        self.delayed.insert(num, count);
        let small_top_val = self.small.peek().unwrap().to_owned();
        if num <= small_top_val {
            self.small_size -= 1;
            if num == small_top_val {
                self.prune_small();
            }
        } else {
            self.large_size -= 1;
            if num == self.large.peek().unwrap().to_owned().0 {
                self.prune_large();
            }
        }
        self.make_balance()
    }

    // 不断地弹出大根heap的堆顶元素，并且更新哈希表
    fn prune_small(&mut self) {
        let mut heap = &mut self.small;
        while !heap.is_empty() {
            let top_num = heap.peek().unwrap().to_owned();
            if self.delayed.contains_key(&top_num) {
                let mut count = self.delayed.get(&top_num).unwrap().to_owned();
                count -= 1;
                if count == 0 {
                    self.delayed.remove(&top_num);
                } else {
                    self.delayed.insert(top_num, count);
                }
                heap.pop();
            } else {
                break;
            }
        }
    }

    // 不断地弹出小根heap的堆顶元素，并且更新哈希表
    fn prune_large(&mut self) {
        let mut heap = &mut self.large;
        while !heap.is_empty() {
            let top_num = heap.peek().unwrap().to_owned().0;
            if self.delayed.contains_key(&top_num) {
                let mut count = self.delayed.get(&top_num).unwrap().to_owned();
                count -= 1;
                if count == 0 {
                    self.delayed.remove(&top_num);
                } else {
                    self.delayed.insert(top_num, count);
                }
                heap.pop();
            } else {
                break;
            }
        }
    }

    // 调整small和large中的元素个数，使得二者的元素个数满足要求
    fn make_balance(&mut self) {
        if self.small_size > self.large_size + 1 {
            // small比large元素多两个
            let small_top_val = self.small.pop().unwrap();
            self.small_size -= 1;
            self.large.push(Reverse(small_top_val));
            self.large_size += 1;
            // small堆顶元素被移除，需要进行prune
            self.prune_small();
        } else if self.small_size < self.large_size {
            // large比small元素多一个
            let large_top_val = self.large.pop().unwrap();
            self.large_size -= 1;
            self.small.push(large_top_val.0);
            self.small_size += 1;
            // large堆顶元素被移除，需要进行prune
            self.prune_large();
        }
    }
}

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let mut my_heap = DualHeap::new(k as usize);
        for i in 0..k as usize {
            my_heap.insert(nums[i])
        }
        let mut result = Vec::new();
        result.push(my_heap.get_median());
        for j in k as usize..nums.len() {
            // 先从窗口中擦除掉第一个元素
            my_heap.erase(nums[j - k as usize]);
            my_heap.insert(nums[j]);
            // 再获取中位数
            result.push(my_heap.get_median());
        }
        result
    }
}
// @lc code=end
