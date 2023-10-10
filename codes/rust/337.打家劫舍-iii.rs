/*
 * @lc app=leetcode.cn id=337 lang=rust
 *
 * [337] 打家劫舍 III
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
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let dp = Self::pri_rob(root.as_ref());
        std::cmp::max(dp[0], dp[1])
    }

    fn pri_rob(root: Option<&Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![0; 2];
        }
        let dp_left = Self::pri_rob(root.clone().unwrap().borrow().left.as_ref());
        let dp_right = Self::pri_rob(root.clone().unwrap().borrow().right.as_ref());
        let mut dp_self = vec![0; 2];
        dp_self[0] = dp_left[1] + dp_right[1] + root.clone().unwrap().borrow().val; // 偷
        dp_self[1] = dp_left[0].max(dp_left[1]) + dp_right[0].max(dp_right[1]); // 不偷
        dp_self
    }
}
// @lc code=end
