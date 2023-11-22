

func hasCycle(head *ListNode) bool {
	// 快慢指针初始化指向 head
	slow, fast := head, head
	// 快指针走到末尾时停止
	for fast != nil && fast.Next != nil {
		// 慢指针走一步，快指针走两步
		slow = slow.Next
		fast = fast.Next.Next
		// 快慢指针相遇，说明含有环
		if slow == fast {
			return true
		}
	}
	// 不包含环
	return false
}