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
        // 定义一个虚拟头节点，prev指针指向虚拟头节点
        let mut dummy = Box::new(ListNode::new(i32::MIN));
        let mut prev = &mut dummy;
        // 定义一个变量用于记录出现重复的值
        let mut duplicate_val = i32::MIN;
        // 顺序遍历链表
        while head.is_some() {
            let mut node = head.unwrap();
            // 探查一下后继节点是否和当前节点重复, 如果重复则将值记录下来
            if let Some(next_node) = node.next.as_ref() {
                if next_node.val == node.val {
                    duplicate_val = node.val;
                }
            }
            head = node.next.take();
            // 如果当前值没有出现重复，则将当前节点接到prev的后面，并更新prev
            if node.val != duplicate_val {
                prev.next = Some(node);
                prev = prev.next.as_mut().unwrap();
            }
        }
        dummy.next
    }
}
// @lc code=end
