

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
		hash = 0
		prime = 29
		firstEntryPower = 1
		map = {}
		sArr = list(s)
		for i in range(length):
			firstEntryPower *= prime
			hash = hash * prime + (ord(sArr[i]) - ord('a'))
		map[hash] = 0
		for i in range(length, len(s)):
			hash = hash * prime + ord(sArr[i]) - ord('a')
			hash -= firstEntryPower * (ord(sArr[i - length]) - ord('a'))
			if hash in map:
				idx = map[hash]
				return s[idx:idx + length]
			map[hash] = i - length + 1
		return ""

	def isDuplicatePresent(self, s: str, length: int) -> bool:
		if length == 0:
			return True
		return self.findDuplicate(s, length) != ""