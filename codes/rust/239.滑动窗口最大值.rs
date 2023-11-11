/*
 * @lc app=leetcode.cn id=239 lang=rust
 *
 * [239] 滑动窗口最大值
 */

// @lc code=start
use std::collections::VecDeque;

struct MonotonicQueue {
    deque: VecDeque<i32>,
}

impl MonotonicQueue {
    fn new() -> Self {
        MonotonicQueue {
            deque: VecDeque::new(),
        }
    }

    fn push(&mut self, val: i32) {
        while let Some(&back) = self.deque.back() {
            if back < val {
                self.deque.pop_back();
            } else {
                break;
            }
        }
        self.deque.push_back(val);
    }

    fn pop(&mut self, val: i32) {
        if let Some(&front) = self.deque.front() {
            if front == val {
                self.deque.pop_front();
            }
        }
    }

    fn max(&self) -> i32 {
        self.deque.front().unwrap().clone()
    }
}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let length = nums.len();
        let mut queue = MonotonicQueue::new();
        for i in 0..length {
            if i < (k - 1) as usize {
                queue.push(nums[i]);
                continue;
            }
            queue.push(nums[i]);
            result.push(queue.max());
            queue.pop(nums[i - (k - 1) as usize]);
        }
        result
    }
}
// @lc code=end
