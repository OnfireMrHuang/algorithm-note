/*
 * @lc app=leetcode.cn id=98 lang=rust
 *
 * [98] 验证二叉搜索树
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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::valid_bst(root, i64::MIN, i64::MAX)
    }

    fn valid_bst(root: Option<Rc<RefCell<TreeNode>>>, lower: i64, upper: i64) -> bool {
        if root.is_none() {
            return true;
        }
        let root_val = root.clone().unwrap().borrow().val;
        if root_val as i64 <= lower || root_val as i64 >= upper {
            return false;
        }
        let left = root.clone().unwrap().borrow().left.clone();
        let right = root.clone().unwrap().borrow().right.clone();
        Self::valid_bst(left, lower, root_val as i64)
            && Self::valid_bst(right, root_val as i64, upper)
    }
}
// @lc code=end
