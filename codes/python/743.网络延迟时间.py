
import heapq
from typing import List

class Solution:
    def networkDelayTime(self, times: List[List[int]], n: int, k: int) -> int:
        # 节点编号是从 1 开始的，所以要一个大小为 n + 1 的邻接表
        graph = [[] for _ in range(n+1)]
        for edge in times:
            from_node, to_node, weight = edge[0], edge[1], edge[2]
            # from -> List<(to, weight)>
            # 邻接表存储图结构，同时存储权重信息
            graph[from_node].append((to_node, weight))
        # 启动 dijkstra 算法计算以节点 k 为起点到其他节点的最短路径
        dist_to = self.dijkstra(k, graph)

        # 找到最长的那一条最短路径
        res = 0
        for i in range(1, len(dist_to)):
            if dist_to[i] == float('inf'):
                # 有节点不可达，返回 -1
                return -1
            res = max(res, dist_to[i])
        return res

    class State:
        # 图节点的 id
        def __init__(self, id: int, dist_from_start: int):
            self.id = id
            # 从 start 节点到当前节点的距离
            self.dist_from_start = dist_from_start

        def __lt__(self, other):
            return self.dist_from_start < other.dist_from_start

    # 输入一个起点 start，计算从 start 到其他节点的最短距离
    def dijkstra(self, start: int, graph: List[List[int]]) -> List[int]:
        # 定义：distTo[i] 的值就是起点 start 到达节点 i 的最短路径权重
        dist_to = [float('inf')] * len(graph)
        # base case，start 到 start 的最短距离就是 0
        dist_to[start] = 0

        # 优先级队列，distFromStart 较小的排在前面
        pq = [Solution.State(start, 0)]
        # 从起点 start 开始进行 BFS
        heapq.heapify(pq)

        while pq:
            cur_state = heapq.heappop(pq)
            cur_node_id = cur_state.id
            cur_dist_from_start = cur_state.dist_from_start

            if cur_dist_from_start > dist_to[cur_node_id]:
                continue

            # 将 cur_node 的相邻节点装入队列
            for neighbor in graph[cur_node_id]:
                next_node_id, dist_to_next_node = neighbor[0], dist_to[cur_node_id] + neighbor[1]
                # 更新 dp table
                if dist_to[next_node_id] > dist_to_next_node:
                    dist_to[next_node_id] = dist_to_next_node
                    heapq.heappush(pq, Solution.State(next_node_id, dist_to_next_node))
        return dist_to