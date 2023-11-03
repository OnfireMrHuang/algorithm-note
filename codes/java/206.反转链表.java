
/**
 * Definition for singly-linked list.
 * public class ListNode {
 * int val;
 * ListNode next;
 * ListNode() {}
 * ListNode(int val) { this.val = val; }
 * ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 * }
 */
class Solution {
	public ListNode reverseList(ListNode head) {
		if (head == null) {
			return null;
		}
		// 定义prev、curr、next三个指针，分别指向前一个节点、当前节点、下一个节点
		ListNode prev = null, curr = head, next = null;
		while (curr != null) {
			next = curr.next;
			// 将curr指针指向prev实现反转
			curr.next = prev;
			// prev、curr指针向后移动
			prev = curr;
			curr = next;
		}
		return prev;
	}
}