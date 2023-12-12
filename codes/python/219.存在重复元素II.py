class Solution:
    def containsNearbyDuplicate(self, nums: List[int], k: int) -> bool:
        dic = {}
        for idx, num in enumerate(nums):
            if num in dic and idx - dic[num] <= k:
                return True
            dic[num] = idx
        return False