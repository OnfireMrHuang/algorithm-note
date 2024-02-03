
import "container/heap"

type MedianFinder struct {
	large *PriorityQueue
	small *PriorityQueue
}

// 小顶堆
type PriorityQueue []int

func (pq PriorityQueue) Len() int {
	return len(pq)
}

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i] < pq[j]
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
}

func (pq *PriorityQueue) Push(x interface{}) {
	*pq = append(*pq, x.(int))
}

func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	x := old[n-1]
	*pq = old[:n-1]
	return x
}

// 大顶堆
type ReversePriorityQueue []int

func (rpq ReversePriorityQueue) Len() int {
	return len(rpq)
}

func (rpq ReversePriorityQueue) Less(i, j int) bool {
	return rpq[i] > rpq[j]
}

func (rpq ReversePriorityQueue) Swap(i, j int) {
	rpq[i], rpq[j] = rpq[j], rpq[i]
}

func (rpq *ReversePriorityQueue) Push(x interface{}) {
	*rpq = append(*rpq, x.(int))
}

func (rpq *ReversePriorityQueue) Pop() interface{} {
	old := *rpq
	n := len(old)
	x := old[n-1]
	*rpq = old[:n-1]
	return x
}

func Constructor() MedianFinder {
	// 小顶堆
	large := &PriorityQueue{}
	// 大顶堆
	small := &ReversePriorityQueue{}

	return MedianFinder{
		large: large,
		small: small,
	}
}

func (this *MedianFinder) FindMedian() float64 {
	// 如果元素不一样多，多的那个堆的堆顶元素就是中位数
	if this.large.Len() < this.small.Len() {
		return float64(this.small.Top())
	} else if this.large.Len() > this.small.Len() {
		return float64(this.large.Top())
	}
	// 如果元素一样多，两个堆堆顶元素的平均数是中位数
	return (float64(this.large.Top()) + float64(this.small.Top())) / 2.0
}

func (this *MedianFinder) AddNum(num int) {
	if this.small.Len() >= this.large.Len() {
		this.small.Push(num)
		heap.Push(this.large, this.small.Pop())
	} else {
		this.large.Push(num)
		heap.Push(this.small, this.large.Pop())
	}
}

// 返回堆顶元素
func (pq *PriorityQueue) Top() int {
	return (*pq)[0]
}

// 返回堆顶元素
func (rpq *ReversePriorityQueue) Top() int {
	return (*rpq)[0]
}