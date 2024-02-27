
import "container/heap"

type Cell struct {
	x int
	y int
	v int
}

type CellHeap []Cell

func (h CellHeap) Len() int {
	return len(h)
}

func (h CellHeap) Less(i, j int) bool {
	return h[i].v < h[j].v
}

func (h CellHeap) Swap(i, j int) {
	h[i], h[j] = h[j], h[i]
}

func (h *CellHeap) Push(x interface{}) {
	*h = append(*h, x.(Cell))
}

func (h *CellHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}

func trapRainWater(heightMap [][]int) int {
	m := len(heightMap)
	n := len(heightMap[0])
	if m <= 2 || n <= 2 {
		return 0
	}
	pq := &CellHeap{}
	heap.Init(pq)
	visit := make([][]bool, m)
	for i := 0; i < m; i++ {
		visit[i] = make([]bool, n)
		for j := 0; j < n; j++ {
			visit[i][j] = false
			if i == 0 || i == m-1 || j == 0 || j == n-1 {
				visit[i][j] = true
				pq.Push(Cell{i, j, heightMap[i][j]})
			}
		}
	}
	result := 0
	directions := []int{-1, 0, 1, 0, -1}
	for pq.Len() > 0 {
		cell := pq.Pop().(Cell)
		for i := 0; i < 4; i++ {
			x := cell.x + directions[i]
			y := cell.y + directions[i+1]
			if x < 0 || x >= m || y < 0 || y >= n || visit[x][y] {
				continue
			}
			if cell.v > heightMap[x][y] {
				result += cell.v - heightMap[x][y]
				heightMap[x][y] = cell.v
			}
			visit[x][y] = true
			pq.Push(Cell{x, y, heightMap[x][y]})
		}
	}
	return result
}