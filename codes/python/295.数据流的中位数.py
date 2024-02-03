
class MedianFinder:
    def __init__(self):
        # 小顶堆
        self.large = []
        # 大顶堆
        self.small = []

    def findMedian(self) -> float:
        # 如果元素不一样多，多的那个堆的堆顶元素就是中位数
        if len(self.large) < len(self.small):
            return float(self.small[0])
        elif len(self.large) > len(self.small):
            return float(self.large[0])
        # 如果元素一样多，两个堆堆顶元素的平均数是中位数
        return (self.large[0] + self.small[0]) / 2.0

    def addNum(self, num: int) -> None:
        if len(self.small) >= len(self.large):
            self.small.append(num)
            self.large.append(-heapq.heappushpop(self.small, -self.large[0]))
        else:
            self.large.append(num)
            self.small.append(-heapq.heappushpop(self.large, -self.small[0]))