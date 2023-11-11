
class Solution:
	def nextGreaterElements(self, nums: List[int]) -> List[int]:
		n = len(nums)
		res = [-1 for _ in range(n)]
		stack = []
		for i in range(n * 2):
			while stack and nums[stack[-1]] < nums[i % n]:
				lastIdx = stack.pop()
				res[lastIdx] = nums[i % n]
			stack.append(i % n)
		return res