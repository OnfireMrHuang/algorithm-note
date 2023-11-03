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

    // 我们可以定义从head开始反转k个节点的函数
    // 反转之后，head就变成了尾，注意需要将尾的后继节点指向第k+1个节点
    fn reverse_head_kth(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        // 因为需要保留head的所有权，方便最后设置后继节点，所以为了避免所有权机制的干扰，我们需要先将head转换为裸指针
        let raw_ptr = Box::into_raw(head.unwrap());

        unsafe {
            // 接着再将裸指针还原，接下来就使用三指针方式反转k个节点
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
