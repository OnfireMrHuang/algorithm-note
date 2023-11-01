/*
 * @lc app=leetcode.cn id=83 lang=rust
 *
 * [83] 删除排序链表中的重复元素
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
        unsafe {
            // 将head转换为指针操作, 并定义快、慢指针指向头节点
            let ptr = Box::into_raw(head.unwrap());

            // 定义一个快、慢指针指标头节点
            let mut fast = ptr;
            let mut slow = ptr;

            // 不断移动快指针，直到快指针为空
            while !fast.is_null() {
                // 如果此时发现慢指针和快指针指向的值不一样，说明有重复的值，
                // 直接将慢指针的next指向快指针来删除掉中间的节点
                if slow.as_ref().unwrap().val != fast.as_ref().unwrap().val {
                    slow.as_mut().unwrap().next = Some(Box::from_raw(fast));
                    slow = fast;
                }
                let next_fast = fast.as_mut().unwrap().next.take();
                if next_fast.is_none() {
                    fast = std::ptr::null_mut();
                } else {
                    fast = Box::into_raw(next_fast.unwrap());
                }
            }
            Some(Box::from_raw(ptr))
        }
    }
}
// @lc code=end
