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
	public ListNode reverseBetween(ListNode head, int left, int right) {
		if (head == null) {
			return head;
		}
		ListNode dummyHead = new ListNode(0);
		dummyHead.next = head;
		// 寻找(a,b)开区间，目标是反转[a,b]闭区间
		ListNode leftPrevNode = dummyHead;
		for (int i = 1; i < left; i++) {
			if (leftPrevNode == null) {
				return dummyHead.next;
			}
			leftPrevNode = leftPrevNode.next;
		}
		ListNode leftNode = leftPrevNode.next;
		leftPrevNode.next = reverseHeadKth(leftNode, right - left);
		return dummyHead.next;
	}

	// 我们可以定义从head开始反转k个节点的函数
	// 反转之后，head就变成了尾，注意需要将尾的后继节点指向第k+1个节点
	private ListNode reverseHeadKth(ListNode head, int k) {
		if (head == null) {
			return head;
		}
		ListNode prev = null;
		ListNode curr = head;
		ListNode next = null;
		for (int i = 0; i <= k; i++) {
			if (curr == null) {
				break;
			}
			next = curr.next;
			curr.next = prev;
			prev = curr;
			curr = next;
		}
		head.next = curr;
		return prev;
	}
}