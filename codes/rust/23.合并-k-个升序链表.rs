/*
 * @lc app=leetcode.cn id=23 lang=rust
 *
 * [23] 合并 K 个升序链表
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
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let len = lists.len();
        if len == 0 {
            return None;
        }

        let mut interval = 1;
        while interval < len {
            for i in (0..len - interval).step_by(interval * 2) {
                lists[i] = Self::merge_two_list(lists[i].take(), lists[i + interval].take());
            }
            interval *= 2;
        }

        lists[0].take()
    }

    fn merge_two_list(
        ans: Option<Box<ListNode>>,
        l: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if ans.is_none() {
            return l;
        }
        if l.is_none() {
            return ans;
        }
        // 转移所有权
        let mut head = ListNode::new(0);
        let mut tail = &mut head;
        let mut left = ans;
        let mut right = l;
        while left.is_some() && right.is_some() {
            let (left_node, right_node) = (left.unwrap(), right.unwrap());
            if left_node.val < right_node.val {
                tail.next = Some(left_node);
                left = tail.next.as_mut().unwrap().next.take();
                right = Some(right_node);
            } else {
                tail.next = Some(right_node);
                left = Some(left_node);
                right = tail.next.as_mut().unwrap().next.take();
            }
            tail = tail.next.as_mut().unwrap();
        }
        tail.next = left.or(right);
        head.next
    }
}
// @lc code=end
