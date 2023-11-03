/*
 * @lc app=leetcode.cn id=148 lang=rust
 *
 * [148] 排序链表
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 空值判断
        if head == None {
            return head;
        }
        let mut list: Vec<Option<Box<ListNode>>> = Vec::new();
        // 第一步: 先将链表中的节点单独取出来，放到一个数组中
        let mut head = head;
        while head.is_some() {
            let next = head.as_mut().unwrap().next.take();
            list.push(head);
            head = next;
        }
        // 第二步: 开始对数组中的链表进行归并排序
        let mut step = 1;
        while step < list.len() {
            let mut i = 0;
            while i + step < list.len() {
                list[i] = Self::merge(list[i].take(), list[i + step].take());
                i += step * 2;
            }
            step *= 2;
        }
        list.get(0).unwrap().to_owned()
    }

    // 合并两个排序好的链表
    fn merge(
        mut head1: Option<Box<ListNode>>,
        mut head2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // 使用一个哨兵节点
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        // 使用尾插法，所以需要尾部结点的引用
        let mut tail = dummy_head.as_mut().unwrap();
        // 如果两个链表都有节点，则继续处理
        while head1.is_some() && head2.is_some() {
            if head1.as_ref().unwrap().val <= head2.as_ref().unwrap().val {
                // 如果head1的值更小，则将head1的头节点取出，放入新的链表中
                let next = head1.as_mut().unwrap().next.take();
                tail.next = head1;
                head1 = next; // 移动到下一个节点
            } else {
                // 如果head2的值更小，则将head2的头节点取出，放到新的链表中
                let next = head2.as_mut().unwrap().next.take();
                tail.next = head2;
                head2 = next;
            }
            tail = tail.next.as_mut().unwrap();
        }
        if head1.is_some() {
            tail.next = head1;
        } else if head2.is_some() {
            tail.next = head2;
        }
        dummy_head.unwrap().next
    }
}
// @lc code=end
