/*
 * @lc app=leetcode.cn id=460 lang=rust
 *
 * [460] LFU 缓存
 */

// @lc code=start

use std::collections::HashMap;
use std::collections::VecDeque;
struct LFUCache {
    capacity: usize,
    kv_map: HashMap<i32, i32>,
    kf_map: HashMap<i32, usize>,
    fk_map: HashMap<usize, VecDeque<i32>>,
    min_freq: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {

    fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            kv_map: HashMap::new(),
            kf_map: HashMap::new(),
            fk_map: HashMap::new(),
            min_freq: 0,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let value = self.kv_map.get(&key);
        if value.is_none() {
            return -1;
        }
        let result = value.unwrap().to_owned();
        self.increase_freq(key);
        result
    }

    fn increase_freq(&mut self, key: i32) {
        // 找到key的频率，对频率进行递增
        let freq = self.kf_map.entry(key).or_insert(0);
        let old_freq = freq.to_owned();
        *freq += 1;

        // 删除原来频率对应key列表中的该key
        let old_key_set = self.fk_map.entry(old_freq).or_insert(VecDeque::new());
        for i in 0..old_key_set.len() {
            if old_key_set[i] == key {
                old_key_set.remove(i);
                break;
            }
        }
        if old_key_set.is_empty() {
            self.fk_map.remove(&old_freq);
            if old_freq == self.min_freq {
                self.min_freq += 1;
            }
        }
        // 往新频率中插入该key
        let new_key_set = self.fk_map.entry(old_freq + 1).or_insert(VecDeque::new());
        new_key_set.push_back(key);
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity <= 0 {
            return;
        }

        // 如果key已经存在，更新value，增加频率
        if self.kv_map.contains_key(&key) {
            self.kv_map.insert(key, value);
            self.increase_freq(key);
            return;
        }

        // 判断容量是否已经满了，如果满了需要淘汰一个key
        if self.kv_map.len() >= self.capacity {
            self.remove_min_freq_key()
        }
        // 增加key的频率
        self.increase_freq(key);
        // 写入kv映射
        self.kv_map.insert(key, value);
        self.min_freq = 1;
    }

    fn remove_min_freq_key(&mut self) {
        // 获取要删除的key
        let key_set = self.fk_map.entry(self.min_freq).or_insert(VecDeque::new());
        let del_key = key_set.front().unwrap().to_owned();
        // 从频率映射key结构中踢出去, 如果该频率没有key了，则
        key_set.pop_front();
        if key_set.is_empty() {
            self.fk_map.remove(&self.min_freq);
            // self.min_freq += 1;
        }
        // 从key到频率的映射中，删除该key
        self.kf_map.remove(&del_key);
        // 从kv存储中移出去
        self.kv_map.remove(&del_key);
    }
}

/**
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
// @lc code=end

