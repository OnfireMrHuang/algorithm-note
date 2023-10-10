/*
 * @lc app=leetcode.cn id=729 lang=rust
 *
 * [729] 我的日程安排表 I
 */

// @lc code=start
use std::collections::BTreeSet;
struct MyCalendar {
    // 存储日程的有序集合, 其中0为日程开始时间, 1为日程结束时间
    booked: BTreeSet<(i32, i32)>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        Self {
            booked: BTreeSet::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        if self.booked.is_empty() {
            self.booked.insert((start, end));
            return true;
        }
        // 获取第一个大于等于start的日程
        if let Some(nxt_rng) = self.booked.range((start, end)..).next() {
            // 如果该日程的开始时间小于end, 则表示该日程与新日程有重叠, 返回false
            if nxt_rng.0 < end {
                return false;
            }
        }
        if let Some(prev_rng) = self.booked.range(..(start, end)).last() {
            // 如果该日程的结束时间大于start, 则表示该日程与新日程有重叠, 返回false
            if prev_rng.1 > start {
                return false;
            }
        }
        self.booked.insert((start, end));
        true
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */
// @lc code=end

