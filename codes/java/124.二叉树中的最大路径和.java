
/**
 * Definition for a binary tree node.
 * public class TreeNode {
 * int val;
 * TreeNode left;
 * TreeNode right;
 * TreeNode() {}
 * TreeNode(int val) { this.val = val; }
 * TreeNode(int val, TreeNode left, TreeNode right) {
 * this.val = val;
 * this.left = left;
 * this.right = right;
 * }
 * }
 */
class Solution {
	private Integer ans = Integer.MIN_VALUE;

	public int maxPathSum(TreeNode root) {
		this.ans = Integer.MIN_VALUE;
		dfs(root);
		return this.ans;
	}

	private int dfs(TreeNode root) {
		if (root == null) {
			return 0;
		}
		int left = Math.max(0, dfs(root.left));
		int right = Math.max(0, dfs(root.right));
		this.ans = Math.max(this.ans, root.val + left + right);
		return root.val + Math.max(left, right);
	}
}