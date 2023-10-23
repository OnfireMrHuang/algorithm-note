from typing import List

class Solution:
	def insert(self, intervals: List[List[int]], newInterval: List[int]) -> List[List[int]]:
		start_int = newInterval[0]
		end_int = newInterval[1]
		result = []
		placed = False
		for interval in intervals:
			if interval[0] > end_int:
				if not placed:
					result.append([start_int, end_int])
					placed = True
				result.append(interval)
			elif interval[1] < start_int:
				result.append(interval)
			else:
				start_int = min(start_int, interval[0])
				end_int = max(end_int, interval[1])
		if not placed:
			result.append([start_int, end_int])
		return result