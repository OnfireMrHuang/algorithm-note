/*
 * @lc app=leetcode.cn id=124 lang=rust
 *
 * [124] 二叉树中的最大路径和
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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = std::i32::MIN;
        Self::priv_max_path_sum(root, &mut ans);
        ans
    }

    fn priv_max_path_sum(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
        if root.is_none() {
            return 0;
        }
        let node = Rc::clone(root.as_ref().unwrap());
        let val = node.borrow().val;
        let left_val = std::cmp::max(0, Self::priv_max_path_sum(node.borrow().left.clone(), ans));
        let right_val = std::cmp::max(0, Self::priv_max_path_sum(node.borrow().right.clone(), ans));

        let mut max_path_sum = ans.to_owned();
        max_path_sum = std::cmp::max(max_path_sum, left_val + right_val + val);
        *ans = max_path_sum;
        std::cmp::max(left_val, right_val) + val
    }
}
// @lc code=end
