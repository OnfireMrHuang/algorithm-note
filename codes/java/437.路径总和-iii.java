import java.util.List;

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
	public int pathSum(TreeNode root, int targetSum) {
		List<Integer> path = new ArrayList<>();
		return dfs(root, targetSum, path);
	}

	private int dfs(TreeNode root, int targetSum, List<Integer> path) {
		if (root == null) {
			return 0;
		}
		path.add(root.val);
		int res = 0;
		long tmp = 0;
		for (int i = path.size() - 1; i >= 0; i--) {
			tmp += path.get(i);
			if (tmp == targetSum) {
				res++;
			}
		}
		res += dfs(root.left, targetSum, path);
		res += dfs(root.right, targetSum, path);
		path.remove(path.size() - 1);
		return res;
	}
}