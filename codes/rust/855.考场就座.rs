/*
 * @lc app=leetcode.cn id=855 lang=rust
 *
 * [855] 考场就座
 */

// @lc code=start
use std::collections::BTreeSet;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
struct Interval {
    l_bound: i32,
    l: i32,
    r: i32,
    r_bound: i32,
}

impl Interval {
    pub fn new(l: i32, l_bound: i32, r: i32, r_bound: i32) -> Self {
        Self {
            l,
            l_bound,
            r,
            r_bound,
        }
    }

    fn distance(&self) -> i32 {
        if self.l == self.l_bound {
            return self.r;
        }
        if self.r == self.r_bound {
            return self.r_bound - 1 - self.l;
        }
        (self.r - self.l) / 2
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_distance = self.distance();
        let other_distence = other.distance();
        if self_distance == other_distence {
            return self.l.cmp(&other.l).reverse();
        }
        self_distance.cmp(&other_distence)
    }
}
struct ExamRoom {
    n: i32,
    set: BTreeSet<Interval>,
    seat_start_interval: HashMap<i32, Interval>,
    seat_end_interval: HashMap<i32, Interval>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ExamRoom {

    fn new(n: i32) -> Self {
        let default_interval = Interval::new(-1, -1, n, n);
        let mut set = BTreeSet::new();
        set.insert(default_interval);
        set.insert(default_interval);
        Self {
            n,
            set,
            seat_start_interval: HashMap::new(),
            seat_end_interval: HashMap::new(),
        }
    }

    fn add_interval(&mut self, interval: Interval) {
        // 往集合里面添加一个区间
        self.set.insert(interval);
        self.seat_start_interval.insert(interval.l, interval);
        self.seat_end_interval.insert(interval.r, interval);
    }

    fn remove_interval(&mut self, interval: Interval) {
        // 从集合里面删除一个区间
        self.set.remove(&interval);
        self.seat_start_interval.remove(&interval.l);
        self.seat_end_interval.remove(&interval.r);
    }

    fn seat(&mut self) -> i32 {
        // 获取有序集合中最大的区间
        let max_interval = self.set.iter().max().unwrap().to_owned();
        let mut seat = 0;
        if max_interval.l == -1 {
            seat = 0;
        } else if max_interval.r == self.n {
            seat = self.n - 1;
        } else {
            // 如果最大区间的左边界不是-1，右边界不是n，那么就是中间的区间，直接返回中间的位置
            seat = max_interval.l + (max_interval.r - max_interval.l) / 2;
        }
        let new_left_interval = Interval::new(max_interval.l, -1, seat, self.n);
        let new_right_interval = Interval::new(seat, -1, max_interval.r, self.n);
        self.remove_interval(max_interval);
        self.add_interval(new_left_interval);
        self.add_interval(new_right_interval);
        seat
    }

    fn leave(&mut self, p: i32) {
        // 获取p的左区间和右区间
        let left_interval = self.seat_end_interval.get(&p).unwrap().to_owned();
        let right_interval = self.seat_start_interval.get(&p).unwrap().to_owned();
        let new_interval = Interval::new(left_interval.l, -1, right_interval.r, self.n);
        self.remove_interval(left_interval);
        self.remove_interval(right_interval);
        self.add_interval(new_interval);
    }
}

/**
 * Your ExamRoom object will be instantiated and called as such:
 * let obj = ExamRoom::new(n);
 * let ret_1: i32 = obj.seat();
 * obj.leave(p);
 */
// @lc code=end

