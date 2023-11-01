
class Solution {
	public ListNode deleteDuplicates(ListNode head) {
		if (head == null) {
			return null;
		}
		ListNode dummy = new ListNode(Integer.MIN_VALUE);
		ListNode prev = dummy;
		int duplicateVal = Integer.MIN_VALUE;
		while (head != null) {
			// 探查一下后继节点是否和当前节点重复, 如果重复则将值记录下来
			if (head.next != null && head.next.val == head.val) {
				duplicateVal = head.val;
			}
			if (head.val != duplicateVal) {
				prev.next = new ListNode(head.val);
				prev = prev.next;
			}
			head = head.next;
		}
		return dummy.next;
	}
}