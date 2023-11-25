
class Solution:
	def twoSum(self, numbers: List[int], target: int) -> List[int]:
		result = [0, 0]
		i = 0
		j = len(numbers) - 1
		while i < j:
			if numbers[i] + numbers[j] == target:
				result[0] = i + 1
				result[1] = j + 1
				break
			if numbers[i] + numbers[j] > target:
				j -= 1
			if numbers[i] + numbers[j] < target:
				i += 1
		return result