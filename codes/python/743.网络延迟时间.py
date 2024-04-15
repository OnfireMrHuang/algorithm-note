import queue
from typing import List


class State():
    def __init__(self, node_id: int, dist_from_start: int):
        self.node_id = node_id
        self.dist_from_start = dist_from_start

    def __lt__(self, other):
        return self.dist_from_start < other.dist_from_start

def custom_compare(item: State) -> int:
    return item.dist_from_start

def dijkstra(graph: List[List[tuple]], start: int) -> List[int]:
    # 定义一个列表，用于存在start节点到每个节点的最短路径
    dist_to_list = [float("inf") for _ in range(len(graph))]
    dist_to_list[start] = 0

    # 定义一个优先级队列， 从拥有最短路径的节点开始遍历
    pq  = queue.PriorityQueue()
    pq.put(State(start, 0))

    # 从队列中取出最短路径的节点，然后更新其相邻节点的最短路径
    while not pq.empty():
        cur_node: State = pq.get()
        cur_node_id = cur_node.node_id
        cur_node_dist_from_start = cur_node.dist_from_start

        # 如果当前节点的路径 > 当前的最短路径，那么就忽略掉
        if cur_node_dist_from_start > dist_to_list[cur_node_id]:
            continue

        # 遍历当前节点的相邻节点，更新其最短路径
        edgs = graph[cur_node_id]
        for edg in edgs:
            next_node_id = edg[0]
            edg_weight = edg[1]
            # 如果当前节点 + 边的权重要比相邻节点的最短路径要小，那么就更新其最短路径, 并将其加入到优先级队列中
            if cur_node_dist_from_start + edg_weight < dist_to_list[next_node_id]:
                dist_to_list[next_node_id] = cur_node_dist_from_start + edg_weight
                pq.put(State(next_node_id, dist_to_list[next_node_id]))

    # 最后返回每个节点的最短路径
    return dist_to_list


class Solution:
    def networkDelayTime(self, times: List[List[int]], n: int, k: int) -> int:
        # 第一步: 首先构造图
        graph = []
        for _ in range(n+1):
            graph.append([])
        for time in times:
            u = time[0]
            v = time[1]
            w = time[2]
            graph[u].append((v, w))
        
        # 第二步: 基于上面的图开始使用dijkstra构造每个节点的最短路径(从start节点到当前节点)
        dist_to_list = dijkstra(graph, k)
        # 第三步: 取其中一个路径最大的节点就是访问完所有节点的最大网络延迟时间
        max_time = 0
        for i in range(1, len(dist_to_list)):
            if dist_to_list[i] == float("inf"):
                return -1
            max_time = max(max_time, dist_to_list[i])
        return max_time            