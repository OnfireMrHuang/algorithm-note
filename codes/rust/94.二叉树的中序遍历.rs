/*
 * @lc app=leetcode.cn id=94 lang=rust
 *
 * [94] 二叉树的中序遍历
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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::traversal(root.as_ref())
    }

    fn traversal(root: Option<&Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();
        if root.is_none() {
            return result;
        }
        let index = root.unwrap().borrow();
        let mut left_vec = Self::traversal(index.left.as_ref());
        if left_vec.len() > 0 {
            result.append(&mut left_vec);
        }
        result.append(&mut vec![index.val]);
        let mut right_vec = Self::traversal(index.right.as_ref());
        if right_vec.len() > 0 {
            result.append(&mut right_vec);
        }
        result
    }
}
// @lc code=end
