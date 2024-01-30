# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def maxPathSum(self, root: Optional[TreeNode]) -> int:
        self.ans = float('-inf')
        self.dfs(root)
        return self.ans


    def dfs(self, root: Optional[TreeNode]) -> int:
        if not root:
            return 0
        left = max(0, self.dfs(root.left))
        right = max(0, self.dfs(root.right))
        self.ans = max(self.ans, root.val + left + right)
        return root.val + max(left, right)
        