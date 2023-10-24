
//Definition for singly-linked list.

func mergeKLists(lists []*ListNode) *ListNode {
	if len(lists) == 0 {
		return nil
	}
	// 虚拟头节点
	dummy := &ListNode{Val: -1}
	p := dummy
	// 优先队列,最小堆, 用golang的heap
	pq := make(Queue, len(lists))
	for i, head := range lists {
		if head != nil {
			pq[i] = head
		}
	}
	heap.Init(&pq)

	for pq.Len() != 0 {
		// 获取最小节点，接到结果链表中
		node := heap.Pop(&pq).(*ListNode)
		p.Next = node
		if node.Next != nil {
			heap.Push(&pq, node.Next)
		}
		// p 指针不断前进
		p = p.Next
	}
	return dummy.Next
}

// golang的堆排序Queue
type Queue []*ListNode

func (q Queue) Len() int { return len(q) }

func (q Queue) Less(i, j int) bool {
	return q[i].Val < q[j].Val
}

func (q Queue) Swap(i, j int) {
	q[i], q[j] = q[j], q[i]
}

func (q *Queue) Push(x interface{}) {
	*q = append(*q, x.(*ListNode))
}

func (q *Queue) Pop() interface{} {
	old := *q
	n := len(old)
	x := old[n-1]
	*q = old[:n-1]
	return x
}