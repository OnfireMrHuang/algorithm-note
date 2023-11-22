
class Solution:
    def detectCycle(self, head: ListNode) -> ListNode:
        fast = slow = head
        while fast and fast.next:
            fast = fast.next.next
            slow = slow.next
            if fast == slow:
                break 
        if not fast or not fast.next:
            return None
        
        slow = head 
        while slow != fast:
            slow = slow.next
            fast = fast.next
        
        return slow