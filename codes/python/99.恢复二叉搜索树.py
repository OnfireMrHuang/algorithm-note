

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
	def recoverTree(self, root: Optional[TreeNode]) -> None:
		"""
		Do not return anything, modify root in-place instead.
		"""
		if not root:
			return
		nums = []
		self.inorder(root, nums)
		x, y = self.find_two_swapped(nums)
		self.recover(root, x, y)
	
	def recover(self, root: Optional[TreeNode], x: int, y: int) -> None:
		if not root:
			return
		if root.val == x:
			root.val = y
		elif root.val == y:
			root.val = x
		self.recover(root.left, x, y)
		self.recover(root.right, x, y)

	def inorder(self, root: Optional[TreeNode], nums: List[int]) -> None:
		if not root:
			return
		self.inorder(root.left, nums)
		nums.append(root.val)
		self.inorder(root.right, nums)

	def find_two_swapped(self, nums: List[int]) -> (int, int):
		x = y = -1
		for i in range(len(nums) - 1):
			if nums[i + 1] < nums[i]:
				y = i+1
				if x == -1:
					x = i
				else:
					break
		return nums[x], nums[y]
	