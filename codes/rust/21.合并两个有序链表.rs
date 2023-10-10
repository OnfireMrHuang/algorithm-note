/*
 * @lc app=leetcode.cn id=21 lang=rust
 *
 * [21] 合并两个有序链表
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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut left = list1;
        let mut right = list2;
        let mut head = ListNode::new(0);
        let mut tail = &mut head;
        while left.is_some() && right.is_some() {
            let (l, r) = (left.take(), right.take());
            if l.as_ref().unwrap().val < r.as_ref().unwrap().val {
                tail.next = l;
                left = tail.next.as_mut().unwrap().next.take();
                right = r;
            } else {
                tail.next = r;
                right = tail.next.as_mut().unwrap().next.take();
                left = l;
            }
            tail = tail.next.as_mut().unwrap();
        }
        tail.next = left.or(right);
        head.next
    }
}
// @lc code=end
