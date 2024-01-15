
class Solution:
	def lengthOfLongestSubstring(self, s: str) -> int:
		max_range_length = 0
		map = {}
		rk = 0
		chars = list(s)
		for i in range(len(chars)):
			if i != 0:
				map.pop(chars[i - 1])
			while rk < len(chars) and chars[rk] not in map:
				map[chars[rk]] = True
				rk += 1
			if rk - i > max_range_length:
				max_range_length = rk - i
		return max_range_length