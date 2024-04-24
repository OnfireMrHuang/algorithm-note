
class Solution:
    def equationsPossible(self, equations: List[str]) -> bool:
        uf = UF(26) #26个字母
        # 先让相等的字母形成连通分量
        for eq in equations:
            if eq[1] == "=":
                x = ord(eq[0]) - ord('a')
                y = ord(eq[3]) - ord('a')
                uf.union(x, y)

        # 检查不等关系是否打破相等关系的连通性
        for eq in equations:
            if eq[1] == "!":
                x = ord(eq[0]) - ord('a')
                y = ord(eq[3]) - ord('a')
                # 如果相等关系成立，就是逻辑冲突
                if uf.connected(x, y):
                    return False
        return True

class UF:
    # 记录连通分量个数
    def __init__(self, n):
        self.count = n
        # 存储若干棵树
        self.parent = [i for i in range(n)]
        # 记录树的“重量”
        self.size = [1] * n

    # 将 p 和 q 连通
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
        self.count -= 1

    # 判断 p 和 q 是否互相连通
    def connected(self, p, q):
        rootP = self.find(p)
        rootQ = self.find(q)
        # 处于同一棵树上的节点，相互连通
        return rootP == rootQ

    # 返回节点 x 的根节点
    def find(self, x):
        while self.parent[x] != x:
            # 进行路径压缩
            self.parent[x] = self.parent[self.parent[x]]
            x = self.parent[x]
        return x

    def count(self):
        return self.count