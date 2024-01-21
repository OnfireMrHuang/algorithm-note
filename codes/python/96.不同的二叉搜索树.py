# 注意：python 代码由 chatGPT🤖 根据我的 java 代码翻译，旨在帮助不同背景的读者理解算法逻辑。
# 本代码已经通过力扣的测试用例，应该可直接成功提交。

class Solution:
    def __init__(self):
        # 备忘录
        self.memo = []
        
    def numTrees(self, n: int) -> int:
        # 备忘录的值初始化为 0
        self.memo = [[0]*(n+1) for _ in range(n+1)]
        return self.count(1, n)

    def count(self, lo: int, hi: int) -> int:
        if lo > hi:
            return 1
        # 查备忘录
        if self.memo[lo][hi] != 0:
            return self.memo[lo][hi]

        res = 0
        for mid in range(lo, hi+1):
            left = self.count(lo, mid - 1)
            right = self.count(mid + 1, hi)
            res += left * right
        # 将结果存入备忘录
        self.memo[lo][hi] = res

        return res