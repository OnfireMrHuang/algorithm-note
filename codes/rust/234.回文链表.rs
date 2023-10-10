/*
 * @lc app=leetcode.cn id=234 lang=rust
 *
 * [234] 回文链表
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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }
        // 使用快慢指针找到链表中点
        let (mut fast, mut slow) = (head.as_ref(), head.as_ref());
        while fast.is_some() && fast.as_ref().unwrap().next.as_ref().is_some() {
            slow = slow.unwrap().next.as_ref();
            fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
        }
        // 反转后面的链表
        let mut prev = None;
        let mut curr = slow.cloned();
        while curr.is_some() {
            let next = curr.as_mut().unwrap().next.take();
            curr.as_mut().unwrap().next = prev.take();
            prev = curr;
            curr = next;
        }
        // 比较前后两个链表
        let mut left = head.as_ref();
        let mut right = prev.as_ref();
        while left.is_some() && right.is_some() {
            if left.as_ref().unwrap().val != right.as_ref().unwrap().val {
                return false;
            }
            left = left.unwrap().next.as_ref();
            right = right.unwrap().next.as_ref();
        }
        true
    }
}
// @lc code=end
