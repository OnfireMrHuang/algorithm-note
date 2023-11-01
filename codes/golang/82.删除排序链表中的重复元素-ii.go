
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func deleteDuplicates(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}
	dummy := &ListNode{Val: -1}
	prev := dummy
	// 设置整型的最小值作为哨兵
	duplicateVal := -1 << 31
	for head != nil {
		if head.Next != nil && head.Next.Val == head.Val {
			duplicateVal = head.Val
		}
		if head.Val != duplicateVal {
			prev.Next = &ListNode{Val: head.Val}
			prev = prev.Next
		}
		head = head.Next
	}
	return dummy.Next
}