
import heapq

class PriorityQueue:
	def __init__(self):
		self.heap = []

	def push(self, item):
		heapq.heappush(self.heap, item)

	def pop(self):
		return heapq.heappop(self.heap)
	
	def is_empty(self):
		return not self.heap
    
	def __lt__(self, other):
		return self[2] < other[2]

class Solution:
	def trapRainWater(self, heightMap: List[List[int]]) -> int:
		m = len(heightMap)
		n = len(heightMap[0])
		if m <= 2 or n <= 2:
			return 0
		pq = PriorityQueue()
		visit = [[False] * n for _ in range(m)]
		for i in range(m):
			for j in range(n):
				if i == 0 or i == m - 1 or j == 0 or j == n - 1:
					visit[i][j] = True
					pq.push((i, j, heightMap[i][j]))
		result = 0
		while not pq.is_empty():
			x, y, v = pq.pop()
			directions = [-1, 0, 1, 0, -1]
			for i in range(4):
				xx = x + directions[i]
				yy = y + directions[i + 1]
				if xx < 0 or xx >= m or yy < 0 or yy >= n or visit[xx][yy]:
					continue
				if v > heightMap[xx][yy]:
					result += v - heightMap[xx][yy]
					heightMap[xx][yy] = v
				visit[xx][yy] = True
				pq.push((xx, yy, heightMap[xx][yy]))
		return result
		

