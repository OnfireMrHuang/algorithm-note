
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func sortList(head *ListNode) *ListNode {
	// 空值判断
	if head == nil {
		return head
	}
	list := make([]*ListNode, 0)
	// 第一步: 先将链表中的节点单独取出来，放到一个数组中
	for head != nil {
		next := head.Next
		head.Next = nil
		list = append(list, head)
		head = next
	}
	// 第二步: 开始对数组中的链表进行归并排序
	step := 1
	for step < len(list) {
		i := 0
		for i+step < len(list) {
			list[i] = merge(list[i], list[i+step])
			i += step * 2
		}
		step *= 2
	}
	return list[0]
}

// 合并两个排序好的链表
func merge(head1, head2 *ListNode) *ListNode {
	// 使用一个哨兵节点
	dummyHead := &ListNode{}
	// 使用尾插法，所以需要尾部结点的引用
	tail := dummyHead
	// 如果两个链表都有节点，则继续处理
	for head1 != nil && head2 != nil {
		if head1.Val <= head2.Val {
			// 如果head1的值更小，则将head1的头节点取出，放入新的链表中
			next := head1.Next
			tail.Next = head1
			head1 = next // 移动到下一个节点
		} else {
			// 如果head2的值更小，则将head2的头节点取出，放到新的链表中
			next := head2.Next
			tail.Next = head2
			head2 = next
		}
		tail = tail.Next
	}
	if head1 != nil {
		tail.Next = head1
	} else if head2 != nil {
		tail.Next = head2
	}
	return dummyHead.Next
}