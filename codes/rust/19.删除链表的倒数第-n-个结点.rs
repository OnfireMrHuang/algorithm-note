/*
 * @lc app=leetcode.cn id=19 lang=rust
 *
 * [19] 删除链表的倒数第 N 个结点
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        dummy.as_mut().unwrap().next = head;
        let mut first = dummy.clone();
        let mut second = dummy.as_mut();

        for _ in 0..=n {
            first = first.unwrap().next.clone();
        }

        while first.is_some() {
            first = first.unwrap().next.clone();
            second = second.unwrap().next.as_mut();
        }

        let mut node_to_remove = second.as_mut().unwrap().next.take();
        second.as_mut().unwrap().next = node_to_remove.as_mut().unwrap().next.take();

        dummy.unwrap().next
    }
}
// @lc code=end
