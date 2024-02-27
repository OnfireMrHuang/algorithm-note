
import "container/heap"

type Item struct {
	word  string
	count int
}

type ItemHeap []Item

func (h ItemHeap) Len() int {
	return len(h)
}

func (h ItemHeap) Less(i, j int) bool {
	if h[i].count == h[j].count {
		return h[i].word > h[j].word
	}
	return h[i].count < h[j].count
}

func (h ItemHeap) Swap(i, j int) {
	h[i], h[j] = h[j], h[i]
}

func (h *ItemHeap) Push(x interface{}) {
	*h = append(*h, x.(Item))
}

func (h *ItemHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}

func topKFrequent(words []string, k int) []string {
	// 构造单词-频率
	mapWordCount := make(map[string]int)
	for _, word := range words {
		mapWordCount[word]++
	}

	// 遍历映射并插入元素到大根堆
	h := &ItemHeap{}
	heap.Init(h)
	for word, count := range mapWordCount {
		heap.Push(h, Item{word, count})
		if h.Len() > k {
			heap.Pop(h)
		}
	}

	result := make([]string, k)
	for i := k - 1; i >= 0; i-- {
		result[i] = heap.Pop(h).(Item).word
	}
	return result
}