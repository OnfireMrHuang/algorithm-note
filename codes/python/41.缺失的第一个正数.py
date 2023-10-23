from typing import List

class Solution:
	def firstMissingPositive(self, nums: List[int]) -> int:
		size = len(nums)
		for i in range(size):
			while nums[i] > 0 and nums[i] <= size and nums[i] != nums[nums[i] - 1]:
				index = nums[i] - 1
				nums[i], nums[index] = nums[index], nums[i]
		for i in range(size):
			if nums[i] != i + 1:
				return i + 1
		return size + 1