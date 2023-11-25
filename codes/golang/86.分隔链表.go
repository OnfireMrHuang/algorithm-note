/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func partition(head *ListNode, x int) *ListNode {

	smlDummy, bigDummy := &ListNode{Val: 0}, &ListNode{Val: 0}
	sml, big := smlDummy, bigDummy

	for head != nil {
		if head.Val < x {
			sml.Next = head
			sml = sml.Next
		} else {
			big.Next = head
			big = big.Next
		}
		head = head.Next
	}

	sml.Next = bigDummy.Next
	big.Next = nil

	return smlDummy.Next
}
