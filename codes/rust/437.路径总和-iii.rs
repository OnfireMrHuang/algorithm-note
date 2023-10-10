/*
 * @lc app=leetcode.cn id=437 lang=rust
 *
 * [437] 路径总和 III
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
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut path: Vec<i32> = Vec::new();
        Self::traverse(root.as_ref(), &mut path, target_sum)
    }

    // 前序遍历
    fn traverse(root: Option<&Rc<RefCell<TreeNode>>>, path: &mut Vec<i32>, target_sum: i32) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut count = 0;
        let root_node = Rc::clone(root.unwrap());
        // 反向遍历路径，对等于目标和的路径计数
        let mut path_sum = root_node.borrow().val as i64;
        if path_sum == target_sum as i64 {
            count += 1;
        }
        for i in (0..path.len()).rev() {
            path_sum += path[i] as i64;
            if path_sum == target_sum as i64 {
                count += 1;
            }
        }
        path.push(root_node.borrow().val); // 添加当前值到路径中
        let left_count = Self::traverse(root_node.borrow().left.as_ref(), path, target_sum);
        let right_count = Self::traverse(root_node.borrow().right.as_ref(), path, target_sum);
        path.pop(); // 将当前节点从路径中撤销
        return count + left_count + right_count;
    }
}
// @lc code=end
