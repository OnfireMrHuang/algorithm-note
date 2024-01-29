# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
	def hasPathSum(self, root: Optional[TreeNode], targetSum: int) -> bool:
		# 如果根节点为空，直接返回False
		if not root:
			return False
		# 如果是叶子节点，判断是否等于目标值
		if root.left is None and root.right is None:
			return root.val == targetSum
		# 递归判断左子树和右子树
		left_has_path_sum = self.hasPathSum(root.left, targetSum-root.val)
		right_has_path_sum = self.hasPathSum(root.right, targetSum-root.val)
		return left_has_path_sum or right_has_path_sum