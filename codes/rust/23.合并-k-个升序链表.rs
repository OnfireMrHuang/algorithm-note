/*
 * @lc app=leetcode.cn id=23 lang=rust
 *
 * [23] 合并 K 个升序链表
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug, PartialOrd, Ord)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

use std::cmp::{Ord, PartialOrd, Reverse};
use std::collections::BinaryHeap;

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.val.cmp(&other.val))
    }
}

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let len = lists.len();
        if len == 0 {
            return None;
        }
        let mut dummy = Box::new(ListNode::new(0));
        let mut p = &mut dummy;
        let mut heap = BinaryHeap::new();
        for i in 0..len {
            if lists[i].is_some() {
                heap.push(Reverse(lists[i].take().unwrap()));
            }
        }
        while !heap.is_empty() {
            let node = heap.pop().unwrap().0;
            p.next = Some(Box::new(ListNode::new(node.val)));
            p = p.next.as_mut().unwrap();
            if node.next.is_some() {
                heap.push(Reverse(node.next.unwrap()));
            }
        }
        dummy.next
    }
}
// @lc code=end
