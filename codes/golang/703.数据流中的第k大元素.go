/*
 * @lc app=leetcode.cn id=703 lang=golang
 *
 * [703] 数据流中的第K大元素
 */

// @lc code=start

import (
	"container/heap"
)

type IntHeap []int

func (h IntHeap) Len() int           { return len(h) }
func (h IntHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h IntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *IntHeap) Push(x interface{}) {
	// Push and Pop use pointer receivers because they modify the slice's length,
	// not just its contents.
	*h = append(*h, x.(int))
}

func (h *IntHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

type KthLargest struct {
	Q IntHeap
	K int
}

func Constructor(k int, nums []int) KthLargest {

	h := IntHeap{}
	h = append(h,nums...)

	// 创建一个最小堆
	heap.Init(&h)
	for i:=k;i<len(nums);i++{
		heap.Pop(&h)
	}
	return KthLargest{
		Q:h,
		K:k,
	}
}

func (this *KthLargest) Add(val int) int {

	heap.Push(&this.Q, val)
	if(this.Q.Len() > this.K) {
		heap.Pop(&this.Q)
	}
	return this.Q[0]
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * obj := Constructor(k, nums);
 * param_1 := obj.Add(val);
 */
// @lc code=end

