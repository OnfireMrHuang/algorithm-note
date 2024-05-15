class Trie:

	def __init__(self):
		self.childrens = [None] * 26
		self.is_end = False

	def insert(self, word: str) -> None:
		node = self
		for ch in word:
			index = ord(ch) - ord('a')
			if node.childrens[index] is None:
				node.childrens[index] = Trie()
			node = node.childrens[index]
		node.is_end = True

	def search(self, word: str) -> bool:
		node = self.searchPrefix(word)
		return node is not None and node.is_end

	def startsWith(self, prefix: str) -> bool:
		return self.searchPrefix(prefix) is not None

	def searchPrefix(self, prefix: str) -> 'Trie':
		node = self
		for ch in prefix:
			index = ord(ch) - ord('a')
			if node.childrens[index] is None:
				return None
			node = node.childrens[index]
		return node



# Your Trie object will be instantiated and called as such:
# obj = Trie()
# obj.insert(word)
# param_2 = obj.search(word)
# param_3 = obj.startsWith(prefix)