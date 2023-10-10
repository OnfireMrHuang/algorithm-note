/*
 * @lc app=leetcode.cn id=113 lang=rust
 *
 * [113] 路径总和 II
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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut path_table: Vec<Vec<i32>> = Vec::new();
        let mut path: Vec<i32> = vec![];
        let target_sub = target_sum;
        Self::dfs(root.as_ref(), &mut path_table, &mut path, target_sub);
        path_table
    }

    fn dfs(
        root: Option<&Rc<RefCell<TreeNode>>>,
        path_table: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
        mut target_sub: i32,
    ) {
        if root.is_none() {
            return;
        }
        let node = root.as_ref().unwrap().borrow();
        target_sub = target_sub - node.val;
        path.push(node.val);
        if node.left.is_none() && node.right.is_none() && target_sub == 0 {
            path_table.push(path.clone());
            return;
        }
        if node.left.is_some() {
            Self::dfs(node.left.as_ref(), path_table, path, target_sub);
            let _ = path.pop();
        }
        if node.right.is_some() {
            Self::dfs(node.right.as_ref(), path_table, path, target_sub);
            let _ = path.pop();
        }
    }
}
// @lc code=end
