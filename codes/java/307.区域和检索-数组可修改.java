
class SegmentTreeNode {
	int left = -1;
	int right = -1;
	int val = 0;
}

class NumArray {

	private int[] nums;
	private int size;
	private SegmentTreeNode[] tree;

	public NumArray(int[] nums) {
		this.nums = nums;
		this.size = nums.length;
		this.tree = new SegmentTreeNode[size * 4];
		build(0, 0, size - 1);
	}

	public void update(int index, int val) {
		if (index < 0) {
			return;
		}
		nums[index] = val;
		updateRecursion(index, val, 0, 0, size - 1);
	}

	private void updateRecursion(int index, int val, int nodeIndex, int nodeLeft, int nodeRight) {
		if (nodeLeft == nodeRight) {
			if (index == nodeLeft) {
				tree[nodeIndex].val = val;
			}
			return;
		}
		int mid = nodeLeft + (nodeRight - nodeLeft) / 2;
		int leftIndex = nodeIndex * 2 + 1;
		int rightIndex = nodeIndex * 2 + 2;
		if (index <= mid) {
			updateRecursion(index, val, leftIndex, nodeLeft, mid);
		} else {
			updateRecursion(index, val, rightIndex, mid + 1, nodeRight);
		}
		tree[nodeIndex].val = tree[leftIndex].val + tree[rightIndex].val;
	}

	public int sumRange(int left, int right) {
		return sumRangeRecursion(left, right, 0, 0, size - 1);
	}

	private int sumRangeRecursion(int queryLeft, int queryRight, int startIndex, int nodeLeft, int nodeRight) {
		if (queryLeft > nodeRight || queryRight < nodeLeft) {
			return 0;
		}
		if (queryLeft <= nodeLeft && queryRight >= nodeRight) {
			return tree[startIndex].val;
		}
		int mid = nodeLeft + (nodeRight - nodeLeft) / 2;
		int leftIndex = startIndex * 2 + 1;
		int rightIndex = startIndex * 2 + 2;
		int leftSumRange = sumRangeRecursion(queryLeft, queryRight, leftIndex, nodeLeft, mid);
		int rightSumRange = sumRangeRecursion(queryLeft, queryRight, rightIndex, mid + 1, nodeRight);
		return leftSumRange + rightSumRange;
	}

	private void build(int index, int left, int right) {
		// 如果left与right的相等，说明区分被划分到只剩一个元素了，那么此时节点的值就是nums[left]
		if (left == right) {
			tree[index] = new SegmentTreeNode();
			tree[index].left = left;
			tree[index].right = right;
			tree[index].val = nums[left];
			return;
		}
		// 否则就继续递归划分区间
		int mid = left + (right - left) / 2;
		int leftIndex = index * 2 + 1;
		int rightIndex = index * 2 + 2;
		build(leftIndex, left, mid);
		build(rightIndex, mid + 1, right);
		// 然后开始计算节点的值
		tree[index] = new SegmentTreeNode();
		tree[index].left = left;
		tree[index].right = right;
		tree[index].val = tree[leftIndex].val + tree[rightIndex].val;
	}
}

/**
 * Your NumArray object will be instantiated and called as such:
 * NumArray obj = new NumArray(nums);
 * obj.update(index,val);
 * int param_2 = obj.sumRange(left,right);
 */