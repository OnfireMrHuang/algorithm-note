/*
 * @lc app=leetcode.cn id=235 lang=rust
 *
 * [235] 二叉搜索树的最近公共祖先
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
    pub fn lowest_common_ancestor(
        tree: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut root = tree;
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;
        while let Some(node) = root.clone() {
            let ref_node = node.borrow();
            let root_val = ref_node.val;
            if root_val > q_val && root_val > p_val {
                root = ref_node.left.clone();
            } else if root_val < q_val && root_val < p_val {
                root = ref_node.right.clone();
            } else {
                return root;
            }
        }
        None
    }
}
// @lc code=end
