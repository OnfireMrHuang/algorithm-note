/*
 * @lc app=leetcode.cn id=141 lang=rust
 *
 * [142] 环形链表II, 该题目暂时不支持rust,本地调试即可
 */

// @lc code=start

// use std::borrow::Borrow;
// use std::cell::RefCell;
// use std::rc::Rc;
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Rc<RefCell<ListNode>>>,
// }

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Rc<RefCell<ListNode>> {
//         Rc::new(RefCell::new(ListNode { next: None, val }))
//     }
// }
impl Solution {
    fn detect_cycle(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
        let mut result = None;
        // 如果链表为空，则不可能有环
        if head.is_none() {
            return None;
        }
        let mut slow = head.as_ref().cloned();
        let mut fast = (*head.as_ref().cloned().unwrap())
            .borrow()
            .next
            .as_ref()
            .cloned();
        while slow.is_some() && fast.is_some() {
            let slow_node = slow.as_ref().cloned().unwrap();
            let fast_node = fast.as_ref().cloned().unwrap();
            if Rc::ptr_eq(&slow_node, &fast_node) {
                let mut ptr = head.as_ref().cloned().unwrap();
                while !Rc::ptr_eq(&ptr, &slow_node) {
                    ptr = (*slow_node).borrow().next.as_ref().cloned().unwrap();
                }
                result = Some(ptr);
            }
            slow = (*slow_node).borrow().next.as_ref().cloned();
            fast = (*(*fast_node).borrow().next.as_ref().cloned().unwrap())
                .borrow()
                .next
                .as_ref()
                .cloned();
        }
        result
    }
}
// @lc code=end
