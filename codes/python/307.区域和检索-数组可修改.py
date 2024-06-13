
class SegmentTreeNode:
    def __init__(self) -> None:
        self.left = -1
        self.right = -1
        self.val = 0


class NumArray:

    def __init__(self, nums: List[int]):
        self.nums = nums
        self.size = len(nums)
        self.tree = [SegmentTreeNode() for _ in range(self.size * 4)]
        self.build(0, 0, self.size - 1)


    def update(self, index: int, val: int) -> None:
        if index < 1:
            return
        self.nums[index] = val
        self.updateRecursion(index, val, 0, 0, self.size - 1)

    def updateRecursion(self, index : int, val : int, nodeIndex : int, nodeLeft : int, nodeRight : int):
        if nodeLeft == nodeRight:
            if nodeIndex == nodeLeft:
                self.tree[nodeIndex].val = val
            return
        mid = nodeLeft + (nodeRight - nodeLeft) // 2
        leftIndex = nodeIndex * 2 + 1
        rightIndex = nodeIndex * 2 + 2
        if index <= mid:
            self.updateRecursion(index, val, leftIndex, nodeLeft, mid)
        else:
            self.updateRecursion(index, val, rightIndex, mid + 1, nodeRight)
        self.tree[nodeIndex].val = self.tree[leftIndex].val + self.tree[rightIndex].val

    def sumRange(self, left: int, right: int) -> int:
        return self.sumRangeRecursion(left, right, 0, 0, self.size - 1)

    def sumRangeRecursion(self, queryLeft : int, queryRight : int, startIndex : int, nodeLeft : int, nodeRight : int):
        if queryLeft > nodeRight or queryRight < nodeLeft:
            return 0
        if queryLeft <= nodeLeft and queryRight >= nodeRight:
            return self.tree[startIndex].val
        mid = nodeLeft + (nodeRight - nodeLeft) // 2
        left_index = startIndex * 2 + 1
        right_index = startIndex * 2 +2
        left_sum_range = self.sumRangeRecursion(queryLeft, queryRight, left_index, nodeLeft, mid)
        right_sum_range = self.sumRangeRecursion(queryLeft, queryRight, right_index, mid + 1, nodeRight)
        return left_sum_range + right_sum_range

    def build(self, index : int, left : int, right : int):
        if left == right:
            self.tree[index].left = left
            self.tree[index].right = right
            self.tree[index].val = self.nums[left]
            return
        mid = left + (right - left) // 2
        leftIndex = index * 2 + 1
        rightIndex = index * 2 + 2
        self.build(leftIndex, left, mid)
        self.build(rightIndex, mid + 1, right)
        self.tree[index].left = left
        self.tree[index].right = right
        self.tree[index].val = self.tree[leftIndex].val + self.tree[rightIndex].val



# Your NumArray object will be instantiated and called as such:
# obj = NumArray(nums)
# obj.update(index,val)
# param_2 = obj.sumRange(left,right)