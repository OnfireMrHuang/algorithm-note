
class Solution:
	def leastInterval(self, tasks: List[str], n: int) -> int:
		if len(tasks) == 0:
			return 0
		size = len(tasks)
		wordCnt = [0] * 26
		for task in tasks:
			index = ord(task) - ord('A')
			wordCnt[index] += 1
		maxCnt = 1
		for cnt in wordCnt:
			if cnt > maxCnt:
				maxCnt = cnt
		tot = 0
		for cnt in wordCnt:
			if cnt == maxCnt:
				tot += 1
		return max(size, (maxCnt-1)*(n+1)+tot)