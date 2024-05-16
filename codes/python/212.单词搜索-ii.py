
class Trie:
	def __init__(self):
		self.children = [None] * 26
		self.word = None

	def insert(self, word):
		node = self
		for c in word:
			if not node.children[ord(c) - ord('a')]:
				node.children[ord(c) - ord('a')] = Trie()
			node = node.children[ord(c) - ord('a')]
		node.word = word

class Solution:
	def findWords(self, board: List[List[str]], words: List[str]) -> List[str]:
		self.resultSet = set()
		self.visited = [[False] * len(board[0]) for _ in range(len(board))]
		self._board = board
		self.directions = [(-1, 0), (1, 0), (0, -1), (0, 1)]

		root = Trie()
		for word in words:
			root.insert(word)

		for i in range(len(board)):
			for j in range(len(board[0])):
				index = ord(board[i][j]) - ord('a')
				if not root.children[index]:
					continue
				self.visited[i][j] = True
				self.dfs(i, j, root.children[index])
				self.visited[i][j] = False
		return list(self.resultSet)

	def dfs(self, i, j, root):
		if root.word:
			self.resultSet.add(root.word)
		for direction in self.directions:
			newI = i + direction[0]
			newJ = j + direction[1]
			if newI < 0 or newI >= len(self._board) or newJ < 0 or newJ >= len(self._board[0]):
				continue
			if self.visited[newI][newJ]:
				continue
			index = ord(self._board[newI][newJ]) - ord('a')
			if not root.children[index]:
				continue
			self.visited[newI][newJ] = True
			self.dfs(newI, newJ, root.children[index])
			self.visited[newI][newJ] = False
	

