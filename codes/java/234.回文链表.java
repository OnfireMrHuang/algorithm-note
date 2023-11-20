
class Solution {
	public boolean isPalindrome(ListNode head) {
		// 第一步: 使用快慢指针找到链表中点
		ListNode slow = head, fast = head;
		while (fast != null && fast.next != null) {
			slow = slow.next;
			fast = fast.next.next;
		}

		// 找到中点之后,反转后半部分链表，并对比前后两部分链表的值
		ListNode left = head, right = reverse(slow);
		while (left != null && right != null) {
			if (left.val != right.val) {
				return false;
			}
			left = left.next;
			right = right.next;
		}
		return true;
	}

	ListNode reverse(ListNode head) {
		ListNode prev = null, curr = head;
		while (curr != null) {
			ListNode next = curr.next;
			curr.next = prev;
			prev = curr;
			curr = next;
		}
		return prev;
	}
}