# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
	def pathSum(self, root: Optional[TreeNode], targetSum: int) -> int:
		path = []
		return self.dfs(root, targetSum, path)

	def dfs(self, root, targetSum, path):
		# 如果当前节点为空，直接返回0
		if not root:
			return 0
		count = 0
		path_sum = root.val
		# 首先判断当前节点是否等于目标值，如果等于，count+1
		if path_sum == targetSum:
			count += 1
		# 然后逆序遍历path，判断是否存在和为targetSum的子序列
		for i in range(len(path)-1, -1, -1):
			path_sum += path[i]
			if path_sum == targetSum:
				count += 1
		# 将当前节点加入path
		path.append(root.val)
		# 递归左右子树
		count += self.dfs(root.left, targetSum, path)
		count += self.dfs(root.right, targetSum, path)
		# 回溯时，将当前节点从path中删除
		path.pop()
		return count