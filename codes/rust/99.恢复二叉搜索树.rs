/*
 * @lc app=leetcode.cn id=99 lang=rust
 *
 * [99] 恢复二叉搜索树
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
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if root.is_none() {
            return;
        }
        let mut nums = Vec::new();
        Self::inorder(root, &mut nums);
        let (x, y) = Self::find_two_swapped(nums);
        Self::recover(root, x, y);
    }

    fn recover(root: &mut Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) {
        if root.is_none() {
            return;
        }
        let node = root.as_mut().unwrap();
        if node.borrow().val == x {
            node.borrow_mut().val = y;
        } else if node.borrow().val == y {
            node.borrow_mut().val = x;
        }
        Self::recover(&mut node.borrow_mut().left, x, y);
        Self::recover(&mut node.borrow_mut().right, x, y);
    }

    fn inorder(root: &Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        let node = Rc::clone(root.as_ref().unwrap());
        Self::inorder(&node.borrow().left, nums);
        nums.push(node.borrow().val);
        Self::inorder(&node.borrow().right, nums);
    }

    fn find_two_swapped(nums: Vec<i32>) -> (i32, i32) {
        let (mut index1, mut index2) = (-1, -1);
        for i in 0..nums.len() - 1 {
            if nums[i + 1] < nums[i] {
                index2 = i as i32 + 1;
                if index1 == -1 {
                    index1 = i as i32;
                } else {
                    break;
                }
            }
        }
        (nums[index1 as usize], nums[index2 as usize])
    }
}
// @lc code=end
