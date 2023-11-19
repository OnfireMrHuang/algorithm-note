
class Solution:
	def threeSum(self, nums: List[int]) -> List[List[int]]:
		res = []
		counter = {}
		for value in nums:
			counter[value] = counter.get(value, 0) + 1

		uniqNums = []
		for key in counter:
			uniqNums.append(key)

		uniqNums.sort()

		for i in range(len(uniqNums)):
			if uniqNums[i] * 3 == 0 and counter[uniqNums[i]] >= 3:
				res.append([uniqNums[i], uniqNums[i], uniqNums[i]])
			for j in range(i + 1, len(uniqNums)):
				if uniqNums[i] * 2 + uniqNums[j] == 0 and counter[uniqNums[i]] > 1:
					res.append([uniqNums[i], uniqNums[i], uniqNums[j]])
				if uniqNums[j] * 2 + uniqNums[i] == 0 and counter[uniqNums[j]] > 1:
					res.append([uniqNums[i], uniqNums[j], uniqNums[j]])
				c = 0 - uniqNums[i] - uniqNums[j]
				if c > uniqNums[j] and counter.get(c, 0) > 0:
					res.append([uniqNums[i], uniqNums[j], c])
		return res
