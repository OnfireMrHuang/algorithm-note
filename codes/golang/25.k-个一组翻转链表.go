
func reverseKGroup(head *ListNode, k int) *ListNode {
	if head == nil {
		return nil
	}

	a, b := head, head
	// 找到需要翻转的区间 [a, b)
	for i := 0; i < k; i++ {
		if b == nil {
			return head
		}
		b = b.Next
	}

	// 反转区间内的链表
	newHead := reverse(a, b)
	// 递归将后续链表的区间也翻转，然后再将它链接到新的区间内
	a.Next = reverseKGroup(b, k)

	return newHead
}

// 翻转区间 [a, b)，包头不包尾
func reverse(a, b *ListNode) *ListNode {
	// 初始化三个指针
	pre, cur, nxt := (*ListNode)(nil), a, a

	// 循环将当前节点指向前一个节点，然后将前一个节点和当前节点往后移动
	for cur != b {
		nxt = cur.Next
		cur.Next = pre
		pre, cur = cur, nxt
	}

	// 返回新的头结点
	return pre
}