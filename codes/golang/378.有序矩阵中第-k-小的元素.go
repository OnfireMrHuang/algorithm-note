// import heapq

// class Solution:
// 	def kthSmallest(self, matrix: List[List[int]], k: int) -> int:
// 		n = len(matrix)
// 		pq = [(matrix[i][0], i, 0) for i in range(n)]
// 		heapq.heapify(pq)

// 		for _ in range(k - 1):
// 			num, x, y = heapq.heappop(pq)
// 			if y != n - 1:
// 				heapq.heappush(pq, (matrix[x][y + 1], x, y + 1))
// 		return heapq.heappop(pq)[0]

import "container/heap"

type pair struct {
	num int
	x   int
	y   int
}

type hp []pair

func (h hp) Len() int { return len(h) }

func (h hp) Less(i, j int) bool { return h[i].num < h[j].num }

func (h hp) Swap(i, j int) { h[i], h[j] = h[j], h[i] }

func (h *hp) Push(v interface{}) { *h = append(*h, v.(pair)) }

func (h *hp) Pop() interface{} {
	a := *h
	v := a[len(a)-1]
	*h = a[:len(a)-1]
	return v
}

func kthSmallest(matrix [][]int, k int) int {
	n := len(matrix)
	h := &hp{}
	heap.Init(h)
	for i := 0; i < n; i++ {
		heap.Push(h, pair{matrix[i][0], i, 0})
	}
	for i := 0; i < k-1; i++ {
		cur := heap.Pop(h).(pair)
		if cur.y != n-1 {
			heap.Push(h, pair{matrix[cur.x][cur.y+1], cur.x, cur.y + 1})
		}
	}
	return heap.Pop(h).(pair).num
}