

class Solution:
    def nextGreaterElement(self, nums1: List[int], nums2: List[int]) -> List[int]:
        # 记录 nums2 中每个元素的下一个更大元素
        greater = self.nextGreater(nums2)
        # 转化成映射：元素 x -> x 的下一个最大元素
        greaterMap = {}
        for i in range(len(nums2)):
            greaterMap[nums2[i]] = greater[i]
        # nums1 是 nums2 的子集，所以根据 greaterMap 可以得到结果
        res = []
        for num in nums1:
            res.append(greaterMap[num])
        return res

    # 计算 nums 中每个元素的下一个更大元素
    def nextGreater(self, nums: List[int]) -> List[int]:
        n = len(nums)
        # 存放答案的数组
        res = [-1] * n
        s = []
        # 倒着往栈里放
        for i in range(n - 1, -1, -1):
            # 判定个子高矮
            while s and s[-1] <= nums[i]:
                # 矮个起开，反正也被挡着了。。。
                s.pop()
            # nums[i] 身后的下一个更大元素
            if s:
                res[i] = s[-1]
            s.append(nums[i])
        return res