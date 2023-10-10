/*
 * @lc app=leetcode.cn id=105 lang=rust
 *
 * [105] 从前序与中序遍历序列构造二叉树
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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut inmap: HashMap<i32, usize> = HashMap::new();
        for i in 0..inorder.len() {
            inmap.insert(inorder[i], i);
        }
        Self::priv_build_tree(
            &preorder,
            0,
            preorder.len() as i32 - 1,
            &inorder,
            0,
            inorder.len() as i32 - 1,
            &inmap,
        )
    }

    fn priv_build_tree(
        preorder: &Vec<i32>,
        pre_start: i32,
        pre_end: i32,
        inorder: &Vec<i32>,
        in_start: i32,
        in_end: i32,
        inmap: &HashMap<i32, usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if pre_start > pre_end || in_start > in_end {
            return None;
        }
        // 先序数组的第一个肯定是根节点
        let root_val = preorder[pre_start as usize];
        let mut root = TreeNode::new(root_val);
        let inorder_root_index = inmap.get(&root_val).unwrap().to_owned();
        // 先序数组的第一个元素是根节点，那么在中序数组中找到这个元素，那么这个元素左边的元素就是左子树的元素，右边的元素就是右子树的元素
        let offset = inorder_root_index as i32 - in_start; // 先序列表中root的右节点偏移量
        let (left_pre_start, left_pre_end, left_in_start, left_in_end) = (
            pre_start + 1,
            pre_start + offset,
            in_start,
            inorder_root_index as i32 - 1,
        );
        let (right_pre_start, right_pre_end, right_in_start, right_in_end) = (
            pre_start + offset + 1,
            pre_end,
            inorder_root_index as i32 + 1,
            in_end,
        );
        root.left = Self::priv_build_tree(
            preorder,
            left_pre_start,
            left_pre_end,
            inorder,
            left_in_start,
            left_in_end,
            inmap,
        );
        root.right = Self::priv_build_tree(
            preorder,
            right_pre_start,
            right_pre_end,
            inorder,
            right_in_start,
            right_in_end,
            inmap,
        );

        Some(Rc::new(RefCell::new(root)))
    }
}
// @lc code=end
