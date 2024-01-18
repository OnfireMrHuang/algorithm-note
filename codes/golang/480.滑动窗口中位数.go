

import "container/heap"

// 可以设置为一个大根堆或者一个小根堆
type hp struct {
	sort.IntSlice
	comparator func(int, int) bool
}

func (h hp) Less(i, j int) bool { return h.comparator(h.IntSlice[i], h.IntSlice[j]) }

func (h *hp) Push(v interface{}) { h.IntSlice = append(h.IntSlice, v.(int)) }

func (h *hp) Pop() interface{} {
	v := h.IntSlice[h.Len()-1]
	h.IntSlice = h.IntSlice[:h.Len()-1]
	return v
}

type DualHeap struct {
	// 大根堆，维护较小的一半元素
	small *hp
	// 小根堆，维护较大的一半元素
	large *hp
	// 哈希表，记录延迟删除的元素，key为元素，value为需要删除的次数
	delayed map[int]int
	// small 和 large 当前包含的元素个数，需要扣除被删除的元素
	smallSize, largeSize int
	k                    int
}

func NewDualHeap(k int) *DualHeap {
	return &DualHeap{
		small:   &hp{comparator: func(i, j int) bool { return i > j }},
		large:   &hp{comparator: func(i, j int) bool { return i < j }},
		delayed: map[int]int{},
		k:       k,
	}
}

func (h *DualHeap) prune() {
	for h.small.Len() > 0 {
		num := h.small.IntSlice[0]
		if h.delayed[num] > 0 {
			h.delayed[num]--
			if h.delayed[num] == 0 {
				delete(h.delayed, num)
			}
			heap.Pop(h.small)
		} else {
			break
		}
	}
	for h.large.Len() > 0 {
		num := h.large.IntSlice[0]
		if h.delayed[num] > 0 {
			h.delayed[num]--
			if h.delayed[num] == 0 {
				delete(h.delayed, num)
			}
			heap.Pop(h.large)
		} else {
			break
		}
	}
}

func (h *DualHeap) Insert(num int) {
	if h.small.Len() == 0 || num <= h.small.IntSlice[0] {
		heap.Push(h.small, num)
		h.smallSize++
	} else {
		heap.Push(h.large, num)
		h.largeSize++
	}
	h.makeBalance()
}

func (h *DualHeap) Erase(num int) {
	h.delayed[num]++
	if num <= h.small.IntSlice[0] {
		h.smallSize--
		if num == h.small.IntSlice[0] {
			h.prune()
		}
	} else {
		h.largeSize--
		if num == h.large.IntSlice[0] {
			h.prune()
		}
	}
	h.makeBalance()
}

func (h *DualHeap) GetMedian() float64 {
	if h.k&1 == 1 {
		return float64(h.small.IntSlice[0])
	}
	return float64(h.small.IntSlice[0]+h.large.IntSlice[0]) / 2
}

func (h *DualHeap) makeBalance() {
	if h.smallSize > h.largeSize+1 {
		heap.Push(h.large, heap.Pop(h.small))
		h.smallSize--
		h.largeSize++
		h.prune()
	} else if h.smallSize < h.largeSize {
		heap.Push(h.small, heap.Pop(h.large))
		h.smallSize++
		h.largeSize--
		h.prune()
	}
}

func medianSlidingWindow(nums []int, k int) []float64 {
	dh := NewDualHeap(k)
	for _, num := range nums[:k] {
		dh.Insert(num)
	}
	ans := make([]float64, 1, len(nums)-k+1)
	ans[0] = dh.GetMedian()
	for i := k; i < len(nums); i++ {
		dh.Insert(nums[i])
		dh.Erase(nums[i-k])
		ans = append(ans, dh.GetMedian())
	}
	return ans
}
