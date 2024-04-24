

class UF:
	def __init__(self, n):
		self.count = n
		self.parent = [i for i in range(n)]
		self.size = [1 for _ in range(n)]

	def find(self, x):
		while x != self.parent[x]:
			self.parent[x] = self.parent[self.parent[x]]
			x = self.parent[x]
		return x

	def connected(self, x, y):
		xRoot = self.find(x)
		yRoot = self.find(y)
		return xRoot == yRoot

	def union(self, x, y):
		xRoot = self.find(x)
		yRoot = self.find(y)
		if xRoot == yRoot:
			return
		if self.size[xRoot] > self.size[yRoot]:
			self.parent[yRoot] = xRoot
			self.size[xRoot] += self.size[yRoot]
		else:
			self.parent[xRoot] = yRoot
			self.size[yRoot] += self.size[xRoot]


class Solution:
	def makeConnected(self, n: int, connections: List[List[int]]) -> int:
		uf = UF(n)
		idleConnection = 0
		unconnected = n - 1
		for connected in connections:
			if uf.connected(connected[0], connected[1]):
				idleConnection += 1
			else:
				uf.union(connected[0], connected[1])
				unconnected -= 1
		if idleConnection < unconnected:
			return -1
		return unconnected