/*
 * @lc app=leetcode.cn id=729 lang=rust
 *
 * [729] 我的日程安排表 I
 */

// @lc code=start
use std::collections::BTreeSet;
struct MyCalendar {
    // 存储日程的有序集合, 其中0为日程开始时间, 1为日程结束时间
    // 元组的比较会按照第一个元素的大小进行比较、如果第一个元素相等则比较第二个元素、以此类推...
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
        // 往右找比预定区间大的区间
        if let Some(nxt_rng) = self.booked.range((start, end)..).next() {
            // 如果该区间的开始时间比预定时间要小，说明重叠了，则直接返回false
            if nxt_rng.0 < end {
                return false;
            }
        }
        // 往左找比预定区间小的区间
        if let Some(prev_rng) = self.booked.range(..(start, end)).last() {
            // 如果该区间的结束时间比预定时间要大，说明重叠了，则直接返回false
            if prev_rng.1 > start {
                return false;
            }
        }
        // 没有重叠则插入
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

