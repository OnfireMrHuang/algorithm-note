/*
 * @lc app=leetcode.cn id=449 lang=rust
 *
 * [449] 序列化和反序列化二叉搜索树
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
use std::rc::Rc;
use std::cell::RefCell;
struct Codec {
	
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        self.encode_recursive(root.as_ref())
    }

    fn encode_recursive(&self, root: Option<&Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return "null".to_string();
        }
        let mut result: Vec<String> = Vec::new();
        let root_node = root.unwrap().borrow();
        result.push(root_node.val.to_string());
        let left_enc = self.encode_recursive(root_node.left.as_ref());
        result.push(left_enc);
        let right_enc = self.encode_recursive(root_node.right.as_ref());
        result.push(right_enc);
        result.join(",")
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }
        // 先分割成数组
        let mut arr = data.split(',').collect::<Vec<&str>>();
        self.decode_recursive(&mut arr)
    }

    fn decode_recursive(&self, arr: &mut Vec<&str>) -> Option<Rc<RefCell<TreeNode>>> {
        if arr.is_empty() {
            return None;
        }
        let first = arr.remove(0);
        if first.eq("null") {
            return None;
        }
        let val = first.parse().unwrap();
        let mut root_node = TreeNode::new(val);
        root_node.left = self.decode_recursive(arr);
        root_node.right = self.decode_recursive(arr);
        Some(Rc::new(RefCell::new(root_node)))
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */
// @lc code=end

