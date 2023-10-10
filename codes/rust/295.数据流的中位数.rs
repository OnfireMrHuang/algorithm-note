/*
 * @lc app=leetcode.cn id=295 lang=rust
 *
 * [295] 数据流的中位数
 */

// @lc code=start

use std::cmp::Reverse;
use std::collections::BinaryHeap;
struct MedianFinder {
    left_heap: BinaryHeap<i32>,           // 左边的大顶堆存放中位数左边的列表
    right_heap: BinaryHeap<Reverse<i32>>, // 右边的小顶堆存放中位数右边的列表
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        Self {
            left_heap: BinaryHeap::new(),
            right_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        // 首先判断左列表和右列表的长度
        let left_heap_size = self.left_heap.len();
        let right_heap_size = self.right_heap.len();
        // 如果两边长度相等(即偶数)，则需要将左堆追加一个元素(先将新元素添加到右堆，再把右堆堆顶加到左堆)
        if left_heap_size == right_heap_size {
            self.right_heap.push(Reverse(num));
            self.left_heap.push(self.right_heap.pop().unwrap().0);
        } else {
            // 如果两边长度不相等(即奇数)，则需要将右堆追加一个元素(先将新元素添加到左堆，再把左堆堆顶加到右堆)
            self.left_heap.push(num);
            self.right_heap.push(Reverse(self.left_heap.pop().unwrap()));
        }
    }

    fn find_median(&self) -> f64 {
        let left_heap_size = self.left_heap.len();
        let right_heap_size = self.right_heap.len();
        if left_heap_size == right_heap_size {
            return (self.left_heap.peek().unwrap() + self.right_heap.peek().unwrap().0) as f64
                / 2.0;
        }
        self.left_heap.peek().unwrap().to_owned() as f64
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
// @lc code=end

