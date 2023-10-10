/*
 * @lc app=leetcode.cn id=236 lang=rust
 *
 * [236] 二叉树的最近公共祖先
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
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;
        let result = Self::recurse(root.as_ref(), p_val, q_val);
        if result.is_some() {
            return Some(Rc::new(RefCell::new(TreeNode::new(result.unwrap()))));
        }
        None
    }

    fn recurse(root: Option<&Rc<RefCell<TreeNode>>>, p_val: i32, q_val: i32) -> Option<i32> {
        // 如果root为空，那么直接返回空
        if root.is_none() {
            return None;
        }
        let root_node = Rc::clone(root.unwrap());
        // 如果root等于p或等于q，那么直接返回root节点
        let root_val = root_node.borrow().val;
        if root_val == q_val || root_val == p_val {
            return Some(root_val);
        }
        // 如果都不想等，则检索左右子树判断
        let left = Self::recurse(root_node.borrow().left.as_ref(), p_val, q_val);
        let right = Self::recurse(root_node.borrow().right.as_ref(), p_val, q_val);
        // 如果左等于p，右等于q或者相反，那么当前节点就是最小公共祖先
        if left.is_some() && right.is_some() {
            let left_val = left.unwrap();
            let right_val = right.unwrap();
            if (left_val == p_val && right_val == q_val)
                || (left_val == q_val && right_val == p_val)
            {
                return Some(root_val);
            }
        }
        // 否则就是哪个有值就返回哪一个
        if left.is_some() {
            return left;
        }
        if right.is_some() {
            return right;
        }
        None
    }
}
// @lc code=end
