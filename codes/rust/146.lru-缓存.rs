/*
 * @lc app=leetcode.cn id=146 lang=rust
 *
 * [146] LRU 缓存
 */
// @lc code=start
use std::{collections::HashMap, ptr, usize};
struct LruEntry {
    key: i32,
    value: i32,
    prev: *mut LruEntry,
    next: *mut LruEntry,
}

impl LruEntry {
    fn new(key: i32, value: i32) -> Self {
        LruEntry {
            key,
            value,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 * 链表法(记录key-index的映射): 
 * put: 
 *  - 查询映射是否存在key,如果存在则变更value,不存在则链表头部插入
 *  - 判断链表的长度是否超过capacity，超过则删除链表尾部元素和映射中该key
 */
struct LRUCache {
    map: HashMap<i32, *mut LruEntry>, // 保存键值对
    capacity: usize,
    // 使用辅助的head和tail指针作为链表的头和尾
    head: *mut LruEntry,
    tail: *mut LruEntry,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let cache = LRUCache {
            map: HashMap::new(),
            capacity: capacity as usize,
            head: Box::into_raw(Box::new(LruEntry::new(0, 0))),
            tail: Box::into_raw(Box::new(LruEntry::new(0, 0))),
        };

        // 初始化连接，头尾互指
        unsafe {
            (*cache.head).next = cache.tail;
            (*cache.tail).prev = cache.head;
        }
        cache
    }

    fn put(&mut self, key: i32, value: i32) {
        // 在映射中先查找这个节点
        let node_ptr = self.map.get_mut(&key).map(|node| {
            let node_ptr: *mut LruEntry = *node;
            node_ptr
        });
        match node_ptr {
            Some(node_ptr) => {
                // 如果节点存在，则更新它的值，然后刷新它
                unsafe {
                    (*node_ptr).value = value;
                    self.detach(node_ptr);
                    self.attach(node_ptr);
                }
            }
            None => {
                // 如果节点不存在，则先判断链表是否已满，清理后再插入
                let node = if self.len() == self.cap() {
                    // 已经满了，需要移除最后一条记录清理空间
                    // 通过尾指针得到最后一个节点key
                    let old_key = unsafe { (*(*self.tail).prev).key };
                    // 从map从删除老节点
                    let mut old_node = self.map.remove(&old_key).unwrap();

                    // 重用老节点
                    unsafe {
                        (*old_node).key = key;
                        (*old_node).value = value;
                    }
                    // 删除链表的最后一个元素
                    self.detach(old_node);
                    old_node
                } else {
                    Box::into_raw(Box::new(LruEntry::new(key, value)))
                };

                // 往链表头部插入该节点
                self.attach(node);

                // 往map中插入该节点
                self.map.insert(key, node);
            }
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        let (node_ptr, value) = match self.map.get(&key) {
            Some(&node) => (Some(node), Some(unsafe { (*node).value })),
            None => (None, Some(-1)),
        };
        // 还需要使用node来刷新链表
        match node_ptr {
            Some(node_ptr) => {
                self.detach(node_ptr);
                self.attach(node_ptr);
            }
            None => (),
        }
        value.unwrap()
    }

    // 容量
    fn cap(&self) -> usize {
        self.capacity
    }

    // // 是否为空
    // fn is_empty(&self) -> bool {
    //     self.map.len() == 0
    // }

    // 链表长度
    fn len(&self) -> usize {
        self.map.len()
    }

    // 从链表中删除
    fn detach(&mut self, node: *mut LruEntry) {
        unsafe {
            (*(*node).prev).next = (*node).next;
            (*(*node).next).prev = (*node).prev;
        }
    }

    // 加到链表的头部
    fn attach(&mut self, node: *mut LruEntry) {
        unsafe {
            (*node).next = (*self.head).next;
            (*node).prev = self.head;
            (*self.head).next = node;
            (*(*node).next).prev = node;
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
// @lc code=end

