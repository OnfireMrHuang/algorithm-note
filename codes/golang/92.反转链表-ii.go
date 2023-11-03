
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func reverseBetween(head *ListNode, left int, right int) *ListNode {
	if head == nil {
		return head
	}
	dummyHead := &ListNode{Val: 0}
	dummyHead.Next = head
	leftPrevNode := dummyHead
	for i := 1; i < left; i++ {
		if leftPrevNode == nil {
			return dummyHead.Next
		}
		leftPrevNode = leftPrevNode.Next
	}
	leftNode := leftPrevNode.Next
	leftPrevNode.Next = reverseHeadKth(leftNode, right-left)
	return dummyHead.Next
}

func reverseHeadKth(head *ListNode, k int) *ListNode {
	if head == nil {
		return head
	}

	var prev *ListNode
	curr := head
	var next *ListNode
	for i := 0; i <= k; i++ {
		if curr == nil {
			break
		}
		next = curr.Next
		curr.Next = prev
		prev = curr
		curr = next
	}
	head.Next = curr
	return prev
}