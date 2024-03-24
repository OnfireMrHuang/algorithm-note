
import (
	"container/heap"
	"math"
)

func networkDelayTime(times [][]int, n int, k int) int {
	// 节点编号是从 1 开始的，所以要一个大小为 n + 1 的邻接表
	graph := make([][][]int, n+1)
	for i := 1; i <= n; i++ {
		graph[i] = make([][]int, 0)
	}

	// 构造图
	for _, edge := range times {
		from := edge[0]
		to := edge[1]
		weight := edge[2]
		// from -> [ [to, weight], ... ]
		// 邻接表存储图结构，同时存储权重信息
		graph[from] = append(graph[from], []int{to, weight})
	}

	// 启动 dijkstra 算法计算以节点 k 为起点到其他节点的最短路径
	distTo := dijkstra(k, graph)

	// 找到最长的那一条最短路径
	res := 0
	for i := 1; i < len(distTo); i++ {
		if distTo[i] == math.MaxInt32 {
			// 有节点不可达，返回 -1
			return -1
		}
		res = max(res, distTo[i])
	}
	return res
}

type State struct {
	// 图节点的 id
	id int
	// 从 start 节点到当前节点的距离
	distFromStart int
}

type PriorityQueue []*State

func (pq PriorityQueue) Len() int { return len(pq) }
func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].distFromStart < pq[j].distFromStart
}
func (pq PriorityQueue) Swap(i, j int) { pq[i], pq[j] = pq[j], pq[i] }

func (pq *PriorityQueue) Push(x interface{}) {
	item := x.(*State)
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	old[n-1] = nil // avoid memory leak
	*pq = old[0 : n-1]
	return item
}

// 输入一个起点 start，计算从 start 到其他节点的最短距离
func dijkstra(start int, graph [][][]int) []int {
	// 定义：distTo[i] 的值就是起点 start 到达节点 i 的最短路径权重
	distTo := make([]int, len(graph))
	for i := 1; i < len(graph); i++ {
		distTo[i] = math.MaxInt32
	}
	// base case，start 到 start 的最短距离就是 0
	distTo[start] = 0

	// 优先级队列，distFromStart 较小的排在前面
	pq := make(PriorityQueue, 0)
	heap.Init(&pq)
	// 从起点 start 开始进行 BFS
	heap.Push(&pq, &State{id: start, distFromStart: 0})

	for pq.Len() > 0 {
		curState := heap.Pop(&pq).(*State)
		curNodeID := curState.id
		curDistFromStart := curState.distFromStart

		if curDistFromStart > distTo[curNodeID] {
			continue
		}

		// 将 curNode 的相邻节点装入队列
		for _, neighbor := range graph[curNodeID] {
			nextNodeID := neighbor[0]
			distToNextNode := distTo[curNodeID] + neighbor[1]
			// 更新 dp table
			if distTo[nextNodeID] > distToNextNode {
				distTo[nextNodeID] = distToNextNode
				heap.Push(&pq, &State{id: nextNodeID, distFromStart: distToNextNode})
			}
		}
	}
	return distTo
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}