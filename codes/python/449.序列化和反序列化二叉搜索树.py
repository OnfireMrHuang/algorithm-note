# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

class Codec:

	def serialize(self, root: Optional[TreeNode]) -> str:
		"""Encodes a tree to a single string.
		"""
		if not root:
			return "null"
		return str(root.val) + "," + self.serialize(root.left) + "," + self.serialize(root.right)
		

	def deserialize(self, data: str) -> Optional[TreeNode]:
		"""Decodes your encoded data to tree.
		"""
		data = data.split(",")
		return self.deserialize_helper(data)

	def deserialize_helper(self, data):
		if data[0] == "null":
			data.pop(0)
			return None
		root = TreeNode(data[0])
		data.pop(0)
		root.left = self.deserialize_helper(data)
		root.right = self.deserialize_helper(data)
		return root
        

# Your Codec object will be instantiated and called as such:
# Your Codec object will be instantiated and called as such:
# ser = Codec()
# deser = Codec()
# tree = ser.serialize(root)
# ans = deser.deserialize(tree)
# return ans