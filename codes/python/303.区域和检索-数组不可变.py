

class NumArray:
	def __init__(self, nums: List[int]):
		self.preSumMap = {}
		preSum = 0
		for i in range(len(nums)):
			preSum += nums[i]
			self.preSumMap[i] = preSum


	def sumRange(self, left: int, right: int) -> int:
		return self.preSumMap.get(right, 0) - self.preSumMap.get(left - 1, 0)



# Your NumArray object will be instantiated and called as such:
# obj = NumArray(nums)
# param_1 = obj.sumRange(left,right)