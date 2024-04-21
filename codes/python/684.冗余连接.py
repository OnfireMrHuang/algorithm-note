

class UnionFind:
	def __init__(self):
		self.parent = {}

	def find(self, i):
		while self.parent.get(i) != None:
			i = self.parent.get(i)
		return i

	def union(self, p, q):
		pRoot = self.find(p)
		qRoot = self.find(q)
		if pRoot == qRoot:
			return
		self.parent[qRoot] = pRoot

class Solution:
	def findRedundantConnection(self, edges: List[List[int]]) -> List[int]:
		uf = UnionFind()
		for edg in edges:
			parent = edg[0]
			children = edg[1]
			if uf.find(parent) == uf.find(children):
				return edg
			uf.union(parent, children)
		return []
