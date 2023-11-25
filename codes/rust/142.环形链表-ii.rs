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
fn detect_cycle(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
    let mut result = None;
    // 如果链表为空，则不可能有环
    if head.is_none() {
        return None;
    }
    let mut slow = head.as_ref().cloned();
    let mut fast = head.as_ref().cloned();
    let mut meet_node = None;
    // 首先找到相遇点
    while fast.is_some() && (*fast.as_ref().cloned().unwrap()).borrow().next.is_some() {
        slow = (*slow.unwrap()).borrow().next.as_ref().cloned();
        fast = (*(*fast.unwrap()).borrow().next.as_ref().cloned().unwrap())
            .borrow()
            .next
            .as_ref()
            .cloned();
        let slow_node = slow.as_ref().cloned().unwrap();
        let fast_node = fast.as_ref().cloned().unwrap();
        if Rc::ptr_eq(&slow_node, &fast_node) {
            meet_node = Some(Rc::clone(&slow_node));
            break;
        }
    }

    if meet_node.is_none() {
        return None;
    }

    // 接着再从头节点和相遇点开始遍历，直到相遇
    let mut left = head.as_ref().cloned();
    let mut right = meet_node.as_ref().cloned();
    while left.is_some() && right.is_some() {
        let left_node = left.as_ref().cloned().unwrap();
        let right_node = right.as_ref().cloned().unwrap();
        if Rc::ptr_eq(&left_node, &right_node) {
            result = Some(Rc::clone(&left_node));
            break;
        }
        left = (*left_node).borrow().next.as_ref().cloned();
        right = (*right_node).borrow().next.as_ref().cloned();
    }
    result
}
// @lc code=end
