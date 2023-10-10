/*
 * @lc app=leetcode.cn id=112 lang=rust
 *
 * [112] 路径总和
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
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if root.is_none() {
            return false;
        }
        let root_val = root.as_ref().unwrap().borrow().val;
        let left = root.as_ref().unwrap().borrow().left.clone();
        let right = root.as_ref().unwrap().borrow().right.clone();
        if left.is_none() && right.is_none() && target_sum == root_val {
            return true;
        }
        Self::has_path_sum(left, target_sum - root_val)
            || Self::has_path_sum(right, target_sum - root_val)
    }
}
// @lc code=end
