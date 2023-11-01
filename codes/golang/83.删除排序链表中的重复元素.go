
func deleteDuplicates(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}

	slow, fast := head, head
	for fast != nil {
		if fast.Val != slow.Val {
			// nums[slow] = nums[fast];
			slow.Next = fast
			// slow++;
			slow = slow.Next
		}
		// fast++
		fast = fast.Next
	}
	// 断开与后面重复元素的连接
	slow.Next = nil
	return head
}