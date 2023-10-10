/*
 * @lc app=leetcode.cn id=814 lang=rust
 *
 * [814] 二叉树剪枝
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
    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        let root_node = root.unwrap();
        let mut node = root_node.borrow_mut();
        node.left = Self::prune_tree(node.left.clone());
        node.right = Self::prune_tree(node.right.clone());

        if node.left.is_none() && node.right.is_none() && node.val == 0 {
            return None;
        }
        Some(root_node.clone())
    }
}
// @lc code=end
