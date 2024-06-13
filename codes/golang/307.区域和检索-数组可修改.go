

type SegmentTreeNode struct {
	left  int
	right int
	val   int
}

func NewSegmentTreeNode() *SegmentTreeNode {
	return &SegmentTreeNode{
		left:  -1,
		right: -1,
		val:   0,
	}
}

type NumArray struct {
	nums []int
	size int
	tree []*SegmentTreeNode
}

func Constructor(nums []int) NumArray {
	var size = len(nums)
	var tree = make([]*SegmentTreeNode, size*4)
	var ans = NumArray{
		nums: nums,
		size: size,
		tree: tree,
	}
	ans.build(0, 0, size-1)
	return ans
}

func (this *NumArray) Update(index int, val int) {
	if index < 0 {
		return
	}
	this.nums[index] = val
	this.updateRecursion(index, val, 0, 0, this.size-1)
}

func (this *NumArray) SumRange(left int, right int) int {
	return this.sumRangeRecursion(left, right, 0, 0, this.size-1)
}

func (this *NumArray) updateRecursion(index int, val int, nodeIndex int, nodeLeft int, nodeRight int) {
	if nodeLeft == nodeRight {
		if index == nodeLeft {
			this.tree[nodeIndex].val = val
		}
		return
	}
	mid := nodeLeft + (nodeRight-nodeLeft)/2
	leftIndex := nodeIndex*2 + 1
	rightIndex := nodeIndex*2 + 2
	if index <= mid {
		this.updateRecursion(index, val, leftIndex, nodeLeft, mid)
	} else {
		this.updateRecursion(index, val, rightIndex, mid+1, nodeRight)
	}
	this.tree[nodeIndex].val = this.tree[leftIndex].val + this.tree[rightIndex].val
}

func (this *NumArray) sumRangeRecursion(queryLeft int, queryRight int, startIndex int, nodeLeft int, nodeRight int) int {
	if queryLeft > nodeRight || queryRight < nodeLeft {
		return 0
	}
	if queryLeft <= nodeLeft && queryRight >= nodeRight {
		return this.tree[startIndex].val
	}
	mid := nodeLeft + (nodeRight-nodeLeft)/2
	leftIndex := startIndex*2 + 1
	rightIndex := startIndex*2 + 2
	leftSumRange := this.sumRangeRecursion(queryLeft, queryRight, leftIndex, nodeLeft, mid)
	rightSumRange := this.sumRangeRecursion(queryLeft, queryRight, rightIndex, mid+1, nodeRight)
	return leftSumRange + rightSumRange
}

func (this *NumArray) build(index int, left int, right int) {
	if left == right {
		this.tree[index] = NewSegmentTreeNode()
		this.tree[index].left = left
		this.tree[index].right = right
		this.tree[index].val = this.nums[left]
		return
	}
	mid := left + (right-left)/2
	leftIndex := index*2 + 1
	rightIndex := index*2 + 2
	this.build(leftIndex, left, mid)
	this.build(rightIndex, mid+1, right)
	this.tree[index] = NewSegmentTreeNode()
	this.tree[index].left = left
	this.tree[index].right = right
	this.tree[index].val = this.tree[leftIndex].val + this.tree[rightIndex].val
}

/**
 * Your NumArray object will be instantiated and called as such:
 * obj := Constructor(nums);
 * obj.Update(index,val);
 * param_2 := obj.SumRange(left,right);
 */