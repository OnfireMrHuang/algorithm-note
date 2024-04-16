
class Solution:
    def minCostConnectPoints(self, points: List[List[int]]) -> int:
        n = len(points)
        # 生成所有边及权重
        edges = []
        for i in range(n):
            for j in range(i + 1, n):
                xi, yi = points[i]
                xj, yj = points[j]
                # 用坐标点在 points 中的索引表示坐标点
                edges.append([i, j, abs(xi - xj) + abs(yi - yj)])
        # 将边按照权重从小到大排序
        edges.sort(key=lambda x: x[2])
        # 执行 Kruskal 算法
        mst = 0
        uf = UF(n)
        for edge in edges:
            u, v, weight = edge
            # 若这条边会产生环，则不能加入 mst
            if uf.connected(u, v):
                continue
            # 若这条边不会产生环，则属于最小生成树
            mst += weight
            uf.union(u, v)
        return mst
    
class UF:
    def __init__(self, n):
        # 连通分量个数
        self.count = n
        # 存储一棵树
        self.parent = list(range(n))
        # 记录树的「重量」
        self.size = [1] * n

    # 将节点 p 和节点 q 连通
    def union(self, p, q):
        rootP = self.find(p)
        rootQ = self.find(q)
        if rootP == rootQ:
            return

        # 小树接到大树下面，较平衡
        if self.size[rootP] > self.size[rootQ]:
            self.parent[rootQ] = rootP
            self.size[rootP] += self.size[rootQ]
        else:
            self.parent[rootP] = rootQ
            self.size[rootQ] += self.size[rootP]
        # 两个连通分量合并成一个连通分量
        self.count -= 1

    # 判断节点 p 和节点 q 是否连通
    def connected(self, p, q):
        rootP = self.find(p)
        rootQ = self.find(q)
        return rootP == rootQ

    # 返回节点 x 的连通分量根节点
    def find(self, x):
        while self.parent[x] != x:
            # 进行路径压缩
            self.parent[x] = self.parent[self.parent[x]]
            x = self.parent[x]
        return x

    # 返回图中的连通分量个数
    def count(self):
        return self.count
    
