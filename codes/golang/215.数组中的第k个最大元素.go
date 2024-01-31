
func findKthLargest(nums []int, k int) int {
	// 小顶堆，堆顶是最小元素
	pq := priorityQueue{}
	for _, e := range nums {
		// 每个元素都要过一遍二叉堆
		pq.Push(e)
		// 堆中元素多于 k 个时，删除堆顶元素
		if pq.Len() > k {
			pq.Pop()
		}
	}
	// pq 中剩下的是 nums 中 k 个最大元素，
	return pq.Peek()
}

type priorityQueue []int

func (pq priorityQueue) Len() int {
	return len(pq)
}

func (pq priorityQueue) Less(i, j int) bool {
	// 小顶堆
	return pq[i] < pq[j]
}

func (pq priorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
}

// Push 往 pq 中放元素
func (pq *priorityQueue) Push(x interface{}) {
	*pq = append(*pq, x.(int))
	// 堆化: 向上调整
	pq.up(pq.Len() - 1)
}

// Pop 从 pq 中取元素
func (pq *priorityQueue) Pop() interface{} {
	n := pq.Len() - 1
	pq.Swap(0, n)
	// 堆化: 向下调整
	pq.down(0, n)
	// 返回最后一个元素
	x := (*pq)[n]
	*pq = (*pq)[:n]
	return x
}

// Peek 返回堆顶元素
func (pq priorityQueue) Peek() int {
	return pq[0]
}

// up 向上调整
func (pq *priorityQueue) up(j int) {
	for {
		i := (j - 1) / 2 // parent
		if i == j || !pq.Less(j, i) {
			break
		}
		pq.Swap(i, j)
		j = i
	}
}

// down 向下调整
func (pq *priorityQueue) down(i, n int) {
	for {
		j1 := 2*i + 1          // left child
		if j1 >= n || j1 < 0 { // j1 < 0 after int overflow
			break
		}
		j := j1 // left child
		// 选出左右孩子中较小的那个
		if j2 := j1 + 1; j2 < n && pq.Less(j2, j1) {
			j = j2 // = 2*i + 2  // right child
		}
		// 如果 pq[i] 比 pq[j] 还小，那么就不用调整了
		if !pq.Less(j, i) {
			break
		}
		pq.Swap(i, j)
		i = j
	}
}

