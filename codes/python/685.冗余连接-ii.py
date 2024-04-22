class UnionFind:
    def __init__(self, n: int):
        self.parent = [i for i in range(n)]
        self.rank = [1 for i in range(n)]

    def find(self, x: int) -> int:
        if x != self.parent[x]:
            self.parent[x] = self.find(self.parent[x])
        return self.parent[x]

    def union(self, x: int, y: int) -> bool:
        rootX = self.find(x)
        rootY = self.find(y)
        if rootX == rootY:
            return False
        if self.rank[rootX] < self.rank[rootY]:
            rootX, rootY = rootY, rootX
        self.parent[rootY] = rootX
        self.rank[rootX] += self.rank[rootY]
        return True

class Solution:
    def findRedundantDirectedConnection(self, edges: List[List[int]]) -> List[int]:
        ans = [0, 0]
        inDegress = self.getNodeInDegress(edges)
        for i in range(len(edges) - 1, -1, -1):
            if inDegress[edges[i][1]] == 2:
                if self.isTree(edges, i):
                    ans[0] = edges[i][0]
                    ans[1] = edges[i][1]
                    return ans
        unionFind = UnionFind(len(edges) + 1)
        for edge in edges:
            if not unionFind.union(edge[0], edge[1]):
                ans[0] = edge[0]
                ans[1] = edge[1]
                return ans
        return ans
    
    def isTree(self, edges: List[List[int]], removeIndex: int) -> bool:
        unionFind = UnionFind(len(edges) + 1)
        for i in range(len(edges)):
            if i == removeIndex:
                continue
            if not unionFind.union(edges[i][0], edges[i][1]):
                return False
        return True
    
    def getNodeInDegress(self, edges: List[List[int]]) -> List[int]:
        inDegrees = [0 for i in range(len(edges) + 1)]
        for edge in edges:
            inDegrees[edge[1]] += 1
        return inDegrees
