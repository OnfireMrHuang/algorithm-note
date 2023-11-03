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
	public ListNode sortList(ListNode head) {
		// 空值判断
		if (head == null) {
			return head;
		}
		List<ListNode> list = new ArrayList<>();
		// 第一步: 先将链表中的节点单独取出来，放到一个数组中
		while (head != null) {
			ListNode next = head.next;
			head.next = null;
			list.add(head);
			head = next;
		}
		// 第二步: 开始对数组中的链表进行归并排序
		int step = 1;
		while (step < list.size()) {
			int i = 0;
			while (i + step < list.size()) {
				list.set(i, merge(list.get(i), list.get(i + step)));
				i += step * 2;
			}
			step *= 2;
		}
		return list.get(0);
	}

	// 合并两个排序好的链表
	private ListNode merge(ListNode head1, ListNode head2) {
		// 使用一个哨兵节点
		ListNode dummyHead = new ListNode(0);
		// 使用尾插法，所以需要尾部结点的引用
		ListNode tail = dummyHead;
		// 如果两个链表都有节点，则继续处理
		while (head1 != null && head2 != null) {
			if (head1.val <= head2.val) {
				// 如果head1的值更小，则将head1的头节点取出，放入新的链表中
				ListNode next = head1.next;
				tail.next = head1;
				head1 = next; // 移动到下一个节点
			} else {
				// 如果head2的值更小，则将head2的头节点取出，放到新的链表中
				ListNode next = head2.next;
				tail.next = head2;
				head2 = next;
			}
			tail = tail.next;
		}
		if (head1 != null) {
			tail.next = head1;
		} else if (head2 != null) {
			tail.next = head2;
		}
		return dummyHead.next;
	}
}