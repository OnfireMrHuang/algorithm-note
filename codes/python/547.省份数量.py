class UnionFind:
	def __init__(self, n):
		self.parent = [i for i in range(n)]
		self.size = [1 for _ in range(n)]
		self.count = n

	def find(self, p):
		while p != self.parent[p]:
			self.parent[p] = self.parent[self.parent[p]]
			p = self.parent[p]
		return p

	def union(self, p, q):
		rootP = self.find(p)
		rootQ = self.find(q)
		if rootP == rootQ:
			return
		if self.size[rootP] > self.size[rootQ]:
			self.parent[rootQ] = rootP
			self.size[rootP] += self.size[rootQ]
		else:
			self.parent[rootP] = rootQ
			self.size[rootQ] += self.size[rootP]
		self.count -= 1


class Solution:
	def findCircleNum(self, isConnected: List[List[int]]) -> int:
		uf = UnionFind(len(isConnected))
		for i in range(len(isConnected)):
			for j in range(i + 1, len(isConnected)):
				if isConnected[i][j] == 1:
					uf.union(i, j)
		return uf.count
	
	