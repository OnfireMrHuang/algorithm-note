import heapq

class Cell:
	def __init__(self,x,y,v) -> None:
		self.x = x
		self.y = y
		self.h = v

	def __lt__(self, other):
		return self.h < other.h

class Solution:
	def trapRainWater(self, heightMap: List[List[int]]) -> int:
		m = len(heightMap)
		n = len(heightMap[0])
		if m <= 2 or n <= 2:
			return 0
		pq = []
		visit = [[False]*n for _ in range(m)]
		for i in range(m):
			for j in range(n):
				if i == 0 or i == m-1 or j == 0 or j == n-1:
					visit[i][j] = True
					heapq.heappush(pq, Cell(i, j, heightMap[i][j]))
		result = 0
		directions = [-1, 0, 1, 0, -1]
		while pq:
			cell = heapq.heappop(pq)
			for i in range(4):
				x = cell.x + directions[i]
				y = cell.y + directions[i+1]
				if x < 0 or x >= m or y < 0 or y >= n or visit[x][y]:
					continue
				if cell.h > heightMap[x][y]:
					result += cell.h - heightMap[x][y]
					heightMap[x][y] = cell.h
				visit[x][y] = True
				heapq.heappush(pq, Cell(x, y, heightMap[x][y]))
		return result
		
