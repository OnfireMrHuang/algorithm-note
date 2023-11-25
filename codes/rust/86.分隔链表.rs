impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut left_dummy = Box::new(ListNode::new(0));
        let mut right_dummy = Box::new(ListNode::new(0));
        let mut left = &mut left_dummy;
        let mut right = &mut right_dummy;
        let mut head = head;
        while head.is_some() {
            let mut node = head.unwrap();
            head = node.next.take();
            if node.val < x {
                left.next = Some(node);
                left = left.next.as_mut().unwrap();
            } else {
                right.next = Some(node);
                right = right.next.as_mut().unwrap();
            }
        }
        // 将右边的链表追加的左边的链表中
        left.next = right_dummy.next.take();
        left_dummy.next
    }
}
