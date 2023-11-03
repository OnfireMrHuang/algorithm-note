
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reverseBetween(self, head: Optional[ListNode], left: int, right: int) -> Optional[ListNode]:
            if not head:
                return head
    
            dummyHead = ListNode(0)
            dummyHead.next = head
    
            leftPrevNode = dummyHead
            for i in range(1, left):
                if not leftPrevNode:
                    return dummyHead.next
                leftPrevNode = leftPrevNode.next
    
            leftNode = leftPrevNode.next
            leftPrevNode.next = self.reverseHeadKth(leftNode, right - left)
    
            return dummyHead.next
    
    def reverseHeadKth(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
            if not head:
                return head
            prev = None
            curr = head
            next = None
            for i in range(0, k + 1):
                if not curr:
                    break
                next = curr.next
                curr.next = prev
                prev = curr
                curr = next
            head.next = curr
            return prev