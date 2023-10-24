

class Solution:
    def mergeKLists(self, lists: List[ListNode]) -> ListNode:
        if not lists:
            return None
        # 虚拟头结点
        dummy = ListNode(-1)
        p = dummy
        # 优先级队列，最小堆
        pq = []
        for head in lists:
            if head:
                heapq.heappush(pq, (head.val, id(head), head))
        # 将 k 个链表的头结点加入最小堆
        while pq:
            # 获取最小节点，接到结果链表中
            node = heapq.heappop(pq)[2]
            p.next = node
            if node.next:
                heapq.heappush(pq, (node.next.val, id(node.next), node.next))
            # p 指针不断前进
            p = p.next
        return dummy.next