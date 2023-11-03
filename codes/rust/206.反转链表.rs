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
        // 定义prev、curr、next三个指针，分别指向前一个节点、当前节点、下一个节点
        let (mut prev, mut curr, mut next) = (None, head, None);
        while curr.is_some() {
            next = curr.as_mut().unwrap().next.take();
            // 将curr指针指向prev实现反转
            curr.as_mut().unwrap().next = prev.take();
            // prev、curr指针向后移动
            prev = curr;
            curr = next;
        }
        prev
    }
}
// @lc code=end
