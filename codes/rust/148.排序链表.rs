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
        // 先算出链表长度,时间O(n)
        let mut length = 0;
        let mut curr = &head;
        while let Some(node) = curr {
            length += 1;
            curr = &node.next;
        }
        // 使用哨兵节点，方便后续操作
        let mut head_pre = Some(Box::new(ListNode { val: 0, next: head }));
        // 接着自底向上排序链表并合并
        let mut sub_length = 1;
        while sub_length < length {
            // tail表示已经归并完成的区间的尾节点，初始化为头部指针
            let mut tail = head_pre.as_mut().unwrap();
            // 开始遍历链表
            // 如果还有剩余节点，则需要继续处理
            // 每次找长度为sub_length的两个区间
            // 然后进行归并
            while tail.next.is_some() {
                let mut head1 = tail.next.take();
                let mut tail1 = head1.as_mut().unwrap();
                for _ in 1..sub_length {
                    // 如果下一个节点为空，则说明遍历完
                    if tail1.next.is_none() {
                        break;
                    }
                    tail1 = tail1.next.as_mut().unwrap();
                }
                // 如果第一个区间就包含剩余的全部节点，不用做归并处理，
                // 直接放回链表，然后跳出循环
                if tail1.next.is_none() {
                    tail.next = head1;
                    break;
                }

                // 第一个区间尾巴节点的下一个节点作为第二个区间的头节点
                let mut head2 = tail1.next.take();
                let mut tail2 = head2.as_mut().unwrap();
                for _ in 1..sub_length {
                    // 如果下一个节点为空，则说明遍历完
                    if tail2.next.is_none() {
                        break;
                    }
                    tail2 = tail2.next.as_mut().unwrap();
                }

                // 从第二个区间的尾部和未处理区间断开
                // 先记录未处理区间的头节点，方便复原
                let next = tail2.next.take();

                // 对两个区间开始进行合并
                tail.next = Self::merge(head1, head2);

                // 如果tail还有下一个节点
                while tail.next.is_some() {
                    // 移动tail到下一个节点
                    tail = tail.next.as_mut().unwrap();
                }
                // 将已合并区间和未合并区间连起来，防止链表断裂
                tail.next = next;
            }
            sub_length <<= 1;
        }

        head_pre.unwrap().next
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
