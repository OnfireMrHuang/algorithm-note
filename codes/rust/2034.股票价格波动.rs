/*
 * @lc app=leetcode.cn id=2034 lang=rust
 *
 * [2034] 股票价格波动
 */

// @lc code=start
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(PartialEq, Eq)]
struct HeapVal {
    timestamp: i32,
    price: i32,
}

impl PartialOrd for HeapVal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.price.partial_cmp(&other.price)
    }
}

impl Ord for HeapVal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price.cmp(&other.price)
    }
}

struct StockPrice {
    max_timestamp: i32,                   // 最大时间戳
    time_price_map: HashMap<i32, i32>,    // 时间戳-价格映射
    pq_max: BinaryHeap<HeapVal>,          // 大顶堆(时间戳-价格)
    pq_min: BinaryHeap<Reverse<HeapVal>>, // 小顶堆(时间戳-价格)
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockPrice {
    fn new() -> Self {
        Self {
            max_timestamp: 0,
            time_price_map: HashMap::new(),
            pq_max: BinaryHeap::new(),
            pq_min: BinaryHeap::new(),
        }
    }

    fn update(&mut self, timestamp: i32, price: i32) {
        // 更新最大时间戳
        self.max_timestamp = self.max_timestamp.max(timestamp);
        self.time_price_map.insert(timestamp, price);
        self.pq_max.push(HeapVal { timestamp, price });
        self.pq_min.push(Reverse(HeapVal { timestamp, price }));
    }

    fn current(&self) -> i32 {
        let val = self.time_price_map.get(&self.max_timestamp);
        if val.is_none() {
            return 0;
        }
        val.unwrap().to_owned()
    }

    fn maximum(&mut self) -> i32 {
        loop {
            let top = self.pq_max.peek();
            if top.is_none() {
                return 0;
            }
            let max_val = top.unwrap();
            let real_val = self
                .time_price_map
                .get(&max_val.timestamp)
                .unwrap()
                .to_owned();
            if real_val == max_val.price {
                return real_val;
            }
            self.pq_max.pop();
        }
    }

    fn minimum(&mut self) -> i32 {
        loop {
            let top = self.pq_min.peek();
            if top.is_none() {
                return 0;
            }
            let max_val = top.unwrap();
            let real_val = self
                .time_price_map
                .get(&max_val.0.timestamp)
                .unwrap()
                .to_owned();
            if real_val == max_val.0.price {
                return real_val;
            }
            self.pq_min.pop();
        }
    }
}

/**
 * Your StockPrice object will be instantiated and called as such:
 * let obj = StockPrice::new();
 * obj.update(timestamp, price);
 * let ret_2: i32 = obj.current();
 * let ret_3: i32 = obj.maximum();
 * let ret_4: i32 = obj.minimum();
 */
// @lc code=end

