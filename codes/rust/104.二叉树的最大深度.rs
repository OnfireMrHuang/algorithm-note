/*
 * @lc app=leetcode.cn id=104 lang=rust
 *
 * [104] 二叉树的最大深度
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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        Self::sub_tree_max_depth(root.as_ref())
    }
    fn sub_tree_max_depth(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        let max_depth: i32;
        if root.is_none() {
            return 0;
        }
        let index = root.unwrap().borrow();
        let left_max_depth = Self::sub_tree_max_depth(index.left.as_ref());
        let right_max_depth = Self::sub_tree_max_depth(index.right.as_ref());
        if left_max_depth > right_max_depth {
            max_depth = left_max_depth + 1;
        } else {
            max_depth = right_max_depth + 1;
        }
        max_depth
    }
}
// @lc code=end
