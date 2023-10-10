/*
 * @lc app=leetcode.cn id=206 lang=rust
 *
 * [206] 反转链表
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let (mut prev, mut curr, mut next) = (None, head, None);
        while curr.is_some() {
            next = curr.as_mut().unwrap().next.take();
            curr.as_mut().unwrap().next = prev.take();
            prev = curr;
            curr = next;
        }
        prev
    }
}
// @lc code=end
