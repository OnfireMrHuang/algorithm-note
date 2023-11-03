# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def sortList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head == None:
            return head
        list = []
        while head != None:
            next = head.next
            head.next = None
            list.append(head)
            head = next
        step = 1
        while step < len(list):
            i = 0
            while i + step < len(list):
                list[i] = self.merge(list[i], list[i + step])
                i += step * 2
            step *= 2
        return list[0]
    
    def merge(self, head1, head2):
        dummyHead = ListNode(0)
        tail = dummyHead
        while head1 != None and head2 != None:
            if head1.val <= head2.val:
                next = head1.next
                tail.next = head1
                head1 = next
            else:
                next = head2.next
                tail.next = head2
                head2 = next
            tail = tail.next
        if head1 != None:
            tail.next = head1
        elif head2 != None:
            tail.next = head2
        return dummyHead.next