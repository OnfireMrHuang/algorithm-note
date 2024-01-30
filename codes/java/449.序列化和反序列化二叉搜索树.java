// impl Codec {
//     fn new() -> Self {
//         Self {}
//     }

//     fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
//         self.encode_recursive(root.as_ref())
//     }

//     fn encode_recursive(&self, root: Option<&Rc<RefCell<TreeNode>>>) -> String {
//         if root.is_none() {
//             return "null".to_string();
//         }
//         let mut result: Vec<String> = Vec::new();
//         let root_node = root.unwrap().borrow();
//         result.push(root_node.val.to_string());
//         let left_enc = self.encode_recursive(root_node.left.as_ref());
//         result.push(left_enc);
//         let right_enc = self.encode_recursive(root_node.right.as_ref());
//         result.push(right_enc);
//         result.join(",")
//     }

//     fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
//         if data.is_empty() {
//             return None;
//         }
//         // 先分割成数组
//         let mut arr = data.split(',').collect::<Vec<&str>>();
//         self.decode_recursive(&mut arr)
//     }

//     fn decode_recursive(&self, arr: &mut Vec<&str>) -> Option<Rc<RefCell<TreeNode>>> {
//         if arr.is_empty() {
//             return None;
//         }
//         let first = arr.remove(0);
//         if first.eq("null") {
//             return None;
//         }
//         let val = first.parse().unwrap();
//         let mut root_node = TreeNode::new(val);
//         root_node.left = self.decode_recursive(arr);
//         root_node.right = self.decode_recursive(arr);
//         Some(Rc::new(RefCell::new(root_node)))
//     }
// }

import java.util.List;

/**
 * Definition for a binary tree node.
 * public class TreeNode {
 * int val;
 * TreeNode left;
 * TreeNode right;
 * TreeNode(int x) { val = x; }
 * }
 */
public class Codec {

	// Encodes a tree to a single string.
	public String serialize(TreeNode root) {
		if (root == null)
			return "null";
		List<String> list = new ArrayList<>();
		list.add(String.valueOf(root.val));
		list.add(serialize(root.left));
		list.add(serialize(root.right));
		return String.join(",", list);
	}

	// Decodes your encoded data to tree.
	public TreeNode deserialize(String data) {
		List<String> list = new ArrayList<>(Arrays.asList(data.split(",")));
		return deserialize(list);
	}

	private TreeNode deserialize(List<String> list) {
		if (list.isEmpty())
			return null;
		String first = list.remove(0);
		if (first.equals("null"))
			return null;
		TreeNode root = new TreeNode(Integer.parseInt(first));
		root.left = deserialize(list);
		root.right = deserialize(list);
		return root;
	}
}

// Your Codec object will be instantiated and called as such:
// Codec ser = new Codec();
// Codec deser = new Codec();
// String tree = ser.serialize(root);
// TreeNode ans = deser.deserialize(tree);
// return ans;