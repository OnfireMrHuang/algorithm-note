/*
 * @lc app=leetcode.cn id=92 lang=rust
 *
 * [92] 反转链表 II
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
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let mut dummy_head = Box::new(ListNode::new(0));
        dummy_head.next = head;
        // 寻找(a,b)开区间，目标是反转[a,b]闭区间
        let mut left_prev_node = Some(&mut dummy_head);
        for _ in 1..left {
            if left_prev_node.is_none() {
                return dummy_head.next;
            }
            left_prev_node = left_prev_node.unwrap().next.as_mut();
        }
        let left_node = left_prev_node.as_mut().unwrap().next.take();
        left_prev_node.unwrap().next = Self::reverse_head_kth(left_node, right - left);
        dummy_head.next
    }

    // 我们可以定义反转
    fn reverse_head_kth(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        let raw_ptr = Box::into_raw(head.unwrap());

        unsafe {
            let new_head = Some(Box::from_raw(raw_ptr));
            let (mut prev, mut curr, mut next) = (None, new_head, None);
            for _ in 0..=k as usize {
                if curr.is_none() {
                    break;
                }
                next = curr.as_mut().unwrap().next.take();
                curr.as_mut().unwrap().next = prev.take();
                prev = curr;
                curr = next;
            }
            (*raw_ptr).next = curr;
            prev
        }
    }
}
// @lc code=end
