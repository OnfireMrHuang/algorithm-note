
class Solution:
	def deleteDuplicates(self, head: Optional[ListNode]) -> Optional[ListNode]:
		if not head:
			return None

		dummy = ListNode(-1)
		prev = dummy
		# 设置<-100的值来作为哨兵
		duplicate_val = -1000

		while head:
			if head.next and head.next.val == head.val:
				duplicate_val = head.val
			if head.val != duplicate_val:
				prev.next = ListNode(head.val)
				prev = prev.next
			head = head.next

		return dummy.next