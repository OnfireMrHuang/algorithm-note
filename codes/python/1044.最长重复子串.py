

class Solution:
	def longestDupSubstring(self, s: str) -> str:
		l = 0
		r = len(s) - 1
		while l < r:
			m = l + ((r - l + 1) >> 1)
			if self.isDuplicatePresent(s, m):
				l = m
			else:
				r = m - 1
		return self.findDuplicate(s, l)
	
	def findDuplicate(self, s: str, length: int) -> str:
		prime = 29 # 选取一个质数，因为质数的乘法比较少，所以哈希冲突的概率比较小，这里选择29的原因是因为26个字母加上3个特殊字符，正好是29个
		hash = 0 # 初始化哈希值为0
		power = 1 # 初始化权重为1
		map = {}
		sArr = list(s)
		for i in range(length):
			power *= prime
			hash = hash * prime + (ord(sArr[i]) - ord('a'))
		map[hash] = 0
		for i in range(length, len(s)):
			hash = hash * prime + ord(sArr[i]) - ord('a') - power * (ord(sArr[i - length]) - ord('a'))
			if hash in map:
				idx = map[hash]
				return s[idx:idx + length]
			map[hash] = i - length + 1
		return ""

	def isDuplicatePresent(self, s: str, length: int) -> bool:
		if length == 0:
			return True
		return self.findDuplicate(s, length) != ""