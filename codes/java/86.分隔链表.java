class Solution {
	public ListNode partition(ListNode head, int x) {
        // 新建两个链表
        ListNode smlDummy = new ListNode(0), bigDummy = new ListNode(0);
        // 遍历链表
        ListNode sml = smlDummy, big = bigDummy;
        while (head != null) {
            // 将 < x 的节点加入 sml 节点后
            if (head.val < x) {
                sml.next = head;
                sml = sml.next;
            // 将 >= x 的节点加入 big 节点后
            } else {
                big.next = head;
                big = big.next;
            }
            head = head.next;
        }
        // 拼接两链表
        sml.next = bigDummy.next;
        big.next = null;
        return smlDummy.next;
    }
}

作者：

Krahets 链接：https:// leetcode.cn/problems/partition-list/solutions/2362068/86-fen-ge-lian-biao-shuang-zhi-zhen-qing-hha7/
来源：力扣（LeetCode）著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处
。