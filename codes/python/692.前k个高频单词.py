
import heapq

class Item:
	def __init__(self, word, count):
		self.word = word
		self.count = count

	def __lt__(self, other):
		if self.count == other.count:
			return self.word > other.word
		return self.count < other.count

class Solution:
	def topKFrequent(self, words: List[str], k: int) -> List[str]:
		# 构造单词-频率
		mapWordCount = {}
		for word in words:
			mapWordCount[word] = mapWordCount.get(word, 0) + 1

		# 遍历映射并插入元素到大根堆
		h = []
		heapq.heapify(h)
		for word, count in mapWordCount.items():
			heapq.heappush(h, Item(word, count))
			if len(h) > k:
				heapq.heappop(h)

		result = []
		for i in range(k):
			result.append(heapq.heappop(h).word)
		return result[::-1]
	