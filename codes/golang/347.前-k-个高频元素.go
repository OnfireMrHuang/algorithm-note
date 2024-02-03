
// 用优先级队列解决这道题
func topKFrequent(nums []int, k int) []int {
	// nums 中的元素 -> 该元素出现的频率
	valToFreq := make(map[int]int)
	for _, v := range nums {
		valToFreq[v]++
	}

	// 定义优先队列，队列按照键值对中的值（元素出现频率）从小到大排序。
	pq := priorityQueue{}
	for key, value := range valToFreq {
		pq = pq.Push(MapEntry{key, value})
		if len(pq) > k {
			pq.Pop()
		}
	}

	res := make([]int, k)
	for i := k - 1; i >= 0; i-- {
		// res 数组中存储前 k 个最大元素
		res[i] = pq.Pop().key
	}

	return res
}

// 定义优先级队列
type MapEntry struct {
	key   int
	value int
}
type priorityQueue []MapEntry

func (pq priorityQueue) Len() int      { return len(pq) }
func (pq priorityQueue) Swap(i, j int) { pq[i], pq[j] = pq[j], pq[i] }
func (pq priorityQueue) Less(i, j int) bool {
	return pq[i].value < pq[j].value
}
func (pq priorityQueue) Push(x interface{}) priorityQueue {
	item := x.(MapEntry)
	return append(pq, item)
}
func (pq priorityQueue) Pop() MapEntry {
	item := pq[pq.Len()-1]
	pq = pq[:pq.Len()-1]
	return item
}

// 用计数排序的方法解决这道题
func topKFrequent2(nums []int, k int) []int {
	// nums 中的元素 -> 该元素出现的频率
	valToFreq := make(map[int]int)
	for _, v := range nums {
		valToFreq[v]++
	}

	// 频率 -> 这个频率有哪些元素
	freqToVals := make([][]int, len(nums)+1)
	for val, freq := range valToFreq {
		if _, ok := freqToVals[freq]; !ok {
			freqToVals[freq] = []int{}
		}
		freqToVals[freq] = append(freqToVals[freq], val)
	}

	res := make([]int, 0, k)
	// freqToVals 从后往前存储着出现最多的元素
	for i := len(freqToVals) - 1; i > 0; i-- {
		if len(freqToVals[i]) == 0 {
			continue
		}
		res = append(res, freqToVals[i]...)
		if len(res) >= k {
			res = res[:k]
			break
		}
	}

	return res
}