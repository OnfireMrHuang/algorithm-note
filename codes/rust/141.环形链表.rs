/*
 * @lc app=leetcode.cn id=141 lang=rust
 *
 * [141] 环形链表, 该题目暂时不支持rust,本地调试即可
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
    fn has_cycle(head: Option<Rc<RefCell<ListNode>>>) -> Option<bool> {
        let mut is_cycle = false;
        // 如果链表为空，则不可能有环
        if head.is_none() {
            return Some(false);
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
                is_cycle = true;
                break;
            }
            slow = (*slow_node).borrow().next.as_ref().cloned();
            fast = (*(*fast_node).borrow().next.as_ref().cloned().unwrap())
                .borrow()
                .next
                .as_ref()
                .cloned();
        }
        Some(is_cycle)
    }
}
// @lc code=end
