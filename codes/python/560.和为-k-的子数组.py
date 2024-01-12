
class Solution:
	def subarraySum(self, nums: List[int], k: int) -> int:
		ans = 0
		pre_sum = 0
		pre_sum_cnt_dict = {0: 1} # 默认空数组的前缀和为0，出现1次
		for num in nums:
			cur_sum = pre_sum + num
			target = cur_sum - k
			if target in pre_sum_cnt_dict:
				ans += pre_sum_cnt_dict[target]
			pre_sum_cnt_dict[cur_sum] = pre_sum_cnt_dict.get(cur_sum, 0) + 1
			pre_sum = cur_sum
		return ans
	