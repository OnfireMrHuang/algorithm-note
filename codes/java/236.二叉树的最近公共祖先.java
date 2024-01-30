/**
 * Definition for a binary tree node.
 * public class TreeNode {
 * int val;
 * TreeNode left;
 * TreeNode right;
 * TreeNode(int x) { val = x; }
 * }
 */
class Solution {
	public TreeNode lowestCommonAncestor(TreeNode root, TreeNode p, TreeNode q) {
		if (root == null)
			return null;
		if (root.val == p.val || root.val == q.val)
			return root;
		TreeNode left = lowestCommonAncestor(root.left, p, q); // 左子树中是否包含p或q
		TreeNode right = lowestCommonAncestor(root.right, p, q); // 右子树是否包含p或q
		if (left != null && right != null) // 左右子树都包含p或q
			return root;
		else if (left != null) // 左子树包含p或q
			return left;
		else if (right != null) // 右子树包含p或q
			return right;
		else
			return null;
	}
}