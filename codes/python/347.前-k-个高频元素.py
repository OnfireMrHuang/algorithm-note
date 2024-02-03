
# 用优先级队列解决这道题
class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        # nums 中的元素 -> 该元素出现的频率
        valToFreq = {}
        for v in nums:
            valToFreq[v] = valToFreq.get(v, 0) + 1

        pq = []
        # 按照键值对中的值（元素出现频率）从小到大排序
        for entry in valToFreq.items():
            heapq.heappush(pq, (entry[1], entry[0]))
            if len(pq) > k:
                # 弹出最小元素，维护队列内是 k 个频率最大的元素
                heapq.heappop(pq)

        res = []
        for i in range(k - 1, -1, -1):
            # res 数组中存储前 k 个最大元素
            res.append(heapq.heappop(pq)[1])

        return res[::-1]


