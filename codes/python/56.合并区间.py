
from typing import List

class Solution:
    def merge(self, intervals: List[List[int]]) -> List[List[int]]:
        res = []
        # 按区间的 start 升序排列
        intervals.sort(key=lambda a: a[0])

        res.append(intervals[0])
        for curr in intervals[1:]:
            # res 中最后一个元素的引用
            last = res[-1]
            if curr[0] <= last[1]:
                last[1] = max(last[1], curr[1])
            else:
                # 处理下一个待合并区间
                res.append(curr)
        return res