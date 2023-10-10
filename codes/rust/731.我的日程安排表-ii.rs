/*
 * @lc app=leetcode.cn id=731 lang=rust
 *
 * [731] 我的日程安排表 II
 */

// @lc code=start
use std::collections::BTreeMap;

//线段树说明: https://www.cnblogs.com/RioTian/p/13409694.html

struct MyCalendarTwo {
    map: BTreeMap<i32, i32>,
}

impl MyCalendarTwo {
    fn new() -> Self {
        Self {
            map: Default::default(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        *self.map.entry(start).or_default() += 1;
        *self.map.entry(end).or_default() -= 1;
        if self
            .map
            .iter()
            .fold((0, false), |(acc, ok), (_, v)| (acc + v, ok || acc + v > 2))
            .1
        {
            *self.map.entry(start).or_default() -= 1;
            *self.map.entry(end).or_default() += 1;
            false
        } else {
            true
        }
    }
}

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * let obj = MyCalendarTwo::new();
 * let ret_1: bool = obj.book(start, end);
 */
// @lc code=end

