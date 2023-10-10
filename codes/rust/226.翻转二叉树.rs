/*
 * @lc app=leetcode.cn id=226 lang=rust
 *
 * [226] 翻转二叉树
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let left = root.clone().unwrap().borrow_mut().left.take();
        let right = root.clone().unwrap().borrow_mut().right.take();
        let left = Self::invert_tree(left);
        let right = Self::invert_tree(right);
        root.clone().unwrap().borrow_mut().left = right;
        root.clone().unwrap().borrow_mut().right = left;
        root
    }
}
// @lc code=end
