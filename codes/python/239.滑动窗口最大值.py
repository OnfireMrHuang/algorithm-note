
import collections

class Solution:
    def maxSlidingWindow(self, nums: List[int], k: int) -> List[int]:
        length = len(nums)
        result = []
        deque = collections.deque()
        for i in range(length):
            if i < k - 1:
                while deque and deque[-1] < nums[i]:
                    deque.pop()
                deque.append(nums[i])
                continue
            while deque and deque[-1] < nums[i]:
                deque.pop()
            deque.append(nums[i])
            result.append(deque[0])
            if deque[0] == nums[i - k + 1]:
                deque.popleft()
        return result