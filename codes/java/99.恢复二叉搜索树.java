
import java.util.Arrays;
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
	public void recoverTree(TreeNode root) {
		// 首先，我们利用中序遍历，得到一个递增的数组
		List<Integer> nums = new ArrayList<>();
		inorder(root, nums);
		// 接着，我们找到两个被交换的元素
		int[] swapped = findTwoSwapped(nums);
		// 最后，我们交换这两个元素
		recover(root, swapped[0], swapped[1]);
	}

	public void inorder(TreeNode root, List<Integer> nums) {
		if (root == null) {
			return;
		}
		inorder(root.left, nums);
		nums.add(root.val);
		inorder(root.right, nums);
	}

	public int[] findTwoSwapped(List<Integer> nums) {
		int n = nums.size();
		int index1 = -1, index2 = -1;
		for (int i = 0; i < n - 1; ++i) {
			if (nums.get(i + 1) < nums.get(i)) {
				index2 = i + 1;
				// first swap occurence
				if (index1 == -1) {
					index1 = i;
				} else {
					// second swap occurence
					break;
				}
			}
		}
		return new int[] { nums.get(index1), nums.get(index2) };
	}

	public void recover(TreeNode r, int x, int y) {
		if (r == null) {
			return;
		}
		if (r.val == x || r.val == y) {
			r.val = r.val == x ? y : x;
		}
		recover(r.left, x, y);
		recover(r.right, x, y);
	}
}