
// 主函数
func removeNthFromEnd(head *ListNode, n int) *ListNode {
	// 虚拟头结点
	dummy := &ListNode{Val: -1, Next: head}
	// 删除倒数第 n 个，要先找倒数第 n + 1 个节点
	x := findFromEnd(dummy, n+1)
	// 删掉倒数第 n 个节点
	x.Next = x.Next.Next
	return dummy.Next
}

// 返回链表的倒数第 k 个节点
func findFromEnd(head *ListNode, k int) *ListNode {
	p1, p2 := head, head
	// p1 先走 k 步
	for i := 0; i < k; i++ {
		p1 = p1.Next
	}
	// p1 和 p2 同时走 n - k 步
	for p1 != nil {
		p1 = p1.Next
		p2 = p2.Next
	}
	// p2 现在指向第 n - k 个节点
	return p2
}