

class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        if len(nums) == 0:
            return 0
        slow, fast = 0, 0
        while fast < len(nums):
            if nums[fast] > nums[slow]:
                slow += 1
                # 维护 nums[0..slow] 无重复
                nums[slow] = nums[fast]
            fast += 1
        # 数组长度为索引 + 1
        return slow + 1