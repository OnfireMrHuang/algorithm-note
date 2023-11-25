class Solution:
    def partition(self, head: Optional[ListNode], x: int) -> Optional[ListNode]:
        # 新建两个链表
        sml_dummy, big_dummy = ListNode(0), ListNode(0)
        # 遍历链表
        sml, big = sml_dummy, big_dummy
        while head:
            # 将 < x 的节点加入 sml 节点后
            if head.val < x:
                sml.next = head
                sml = sml.next
            # 将 >= x 的节点加入 big 节点后
            else:
                big.next = head
                big = big.next
            head = head.next
        # 拼接两链表
        sml.next = big_dummy.next
        big.next = None
        return sml_dummy.next
