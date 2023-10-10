/*
 * @lc app=leetcode.cn id=82 lang=rust
 *
 * [82] 删除排序链表中的重复元素 II
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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let mut head = head;
        let mut new_head = Box::new(ListNode::new(i32::MIN));
        let mut ptr = &mut new_head;
        let mut duplicate_val = i32::MIN;
        while head.is_some() {
            let mut node = head.unwrap();
            // 多遍历一次，如果发现有重复值，则记下来，排除所有该数字
            if let Some(next_node) = node.next.as_ref() {
                if next_node.val == node.val {
                    duplicate_val = node.val;
                }
            }
            head = node.next.take();
            if node.val != duplicate_val {
                ptr.next = Some(node);
                ptr = ptr.next.as_mut().unwrap();
            }
        }
        new_head.next
    }
}
// @lc code=end
