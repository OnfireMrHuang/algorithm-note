/*
 * @lc app=leetcode.cn id=732 lang=rust
 *
 * [732] 我的日程安排表 III
 */

// @lc code=start
use std::collections::BTreeMap;

struct MyCalendarThree {
    cache: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarThree {
    fn new() -> Self {
        MyCalendarThree {
            cache: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> i32 {
        self.cache
            .insert(start, self.cache.get(&start).unwrap_or(&0) + 1);
        self.cache
            .insert(end, self.cache.get(&end).unwrap_or(&0) - 1);
        self.cache
            .iter()
            .fold((0, 0), |(sum, mut step), (k, v)| (sum.max(step), step + v))
            .0
    }
}

/**
 * Your MyCalendarThree object will be instantiated and called as such:
 * let obj = MyCalendarThree::new();
 * let ret_1: i32 = obj.book(startTime, endTime);
 */
// @lc code=end

