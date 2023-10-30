/*
 * @lc app=leetcode.cn id=25 lang=rust
 *
 * [25] K 个一组翻转链表
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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // 如果节点为空或者k<=1(即没有节点需要反转)，则直接返回
        if head.is_none() || k <= 1 {
            return head;
        }
        let mut head = head;
        let mut tail = head.as_mut();
        // 从头节点出发，找出k个节点大小的子链表, 不足k个节点的话，直接返回
        for _ in 0..k {
            if tail.is_none() {
                return head;
            }
            tail = tail.unwrap().next.as_mut();
        }
        // 此时以[head, next_head)区间的子链表进行反转，然后返回新的头节点
        let next_head = tail.cloned();
        let (new_head, new_tail) = Self::reverse(head, next_head.as_ref());
        // 如果next_head不为空，则递归反转后面的子链表，将子链表的头加入到当前的尾节点的后面
        if next_head.is_some() {
            unsafe {
                let result = Self::reverse_k_group(next_head, k);
                (*new_tail).next = result;
            }
        }
        new_head
    }

    // 反转区间链表，操作完成之后再返回新的head和tail
    // 这里使用的是[head,next_head)右开区间
    fn reverse(
        mut head: Option<Box<ListNode>>,
        target: Option<&Box<ListNode>>,
    ) -> (Option<Box<ListNode>>, *mut ListNode) {
        let head_heap = head.unwrap();
        let raw_ptr = Box::into_raw(head_heap);
        // 因为涉及到链表的反转，rust中的所有权机制导致代码非常复杂，所以这里直接将头指针转换为裸指针，然后进行操作
        unsafe {
            let new_head = Some(Box::from_raw(raw_ptr));
            // 定义前、中、后三个指针，然后进行反转
            let (mut prev, mut curr, mut next) = (None, new_head, None);
            while curr.as_ref() != target {
                next = curr.as_mut().unwrap().next.take();
                curr.as_mut().unwrap().next = prev;
                prev = curr;
                curr = next;
            }
            (prev, raw_ptr)
        }
    }
}
// @lc code=end
