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
        // println!("xx1: {:?}", head.as_ref());
        if head.is_none() {
            return head;
        }
        if k <= 1 {
            return head;
        }
        let mut head = head;
        let mut idx = head.as_mut();
        for _ in 1..k as usize {
            if idx.is_none() {
                return head;
            }
            idx = idx.unwrap().next.as_mut();
        }
        if idx.is_none() {
            return head;
        }
        let tail = idx.unwrap().next.clone();
        let mut val = -1;
        if tail.is_some() {
            val = tail.as_ref().unwrap().val;
        }

        // 翻转
        // println!("xx2: {:?}", head.as_ref());
        let (new_head, mut new_tail) = Self::reverse(head, tail.as_ref());
        // println!("xx3: {:?}", new_head.as_ref());

        // 递归
        unsafe {
            let result = Self::reverse_k_group(tail, k);
            (*new_tail).next = result;
        }
        new_head
    }

    // 反转区间链表，操作完成之后再返回新的head.这里使用的是[head,tail)右开区间
    fn reverse(
        mut head: Option<Box<ListNode>>,
        target: Option<&Box<ListNode>>,
    ) -> (Option<Box<ListNode>>, *mut ListNode) {
        let head_heap = head.unwrap();
        let raw_ptr = Box::into_raw(head_heap);

        unsafe {
            let new_head = Some(Box::from_raw(raw_ptr));
            // println!("xx21: {:?}", new_head.as_ref());
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
