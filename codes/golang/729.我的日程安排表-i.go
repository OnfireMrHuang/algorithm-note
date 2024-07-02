
type SegmentTreeNode struct {
	Left         int              // 左区间值
	Right        int              // 右区间值
	BookedStatus int              // 0: 区间未被预定， 1: 区间全部被预定， 2: 区间部分被预定
	LeftChild    *SegmentTreeNode // 左子节点
	RightChild   *SegmentTreeNode // 右子节点
}

type MyCalendar struct {
	root *SegmentTreeNode // 根节点
}

func Constructor() MyCalendar {
	// 初始化一个
	var r = &SegmentTreeNode{
		Left:  0,
		Right: 1000000000,
	}
	return MyCalendar{root: r}
}

func (this *MyCalendar) Book(start int, end int) bool {
	var ok = book(this.root, start, end)
	if !ok {
		return false
	}
	// 如果确定能预定，则再将内部相关的线段状态更改
	pushDownStatus(this.root, start, end)
	return true
}

// 节点区间与预定区间都是左闭右开区间
func book(root *SegmentTreeNode, start int, end int) bool {
	// 因为这里节点值是左闭右开区间[a,b),所以针对[1,2)这种区间，只需要判断左值即可
	if root.Left == root.Right-1 {
		return root.Left >= start && root.Left < end
	}
	// 如果节点区间与预定区间无交集, 则直接返回
	if withoutBookRange(root, start, end) {
		return true
	}
	// 如果节点区间完全在预定区间内
	if withInBookRange(root, start, end) {
		var ans bool
		switch root.BookedStatus {
		case 0:
			ans = true
		default:
			// 存在冲突，返回预定失败
			ans = false
		}
		return ans
	}
	// 预定区间存在交集
	var mid = root.Left + (root.Right-root.Left)/2
	root.LeftChild = constructChild(root.LeftChild, root.Left, mid)    // 构造左子树， 注意左闭右开区间
	root.RightChild = constructChild(root.RightChild, mid, root.Right) // 构造右子树， 注意左闭右开区间
	var bookAns = true
	// 如果和左区间有交集，则递归判断一下左区间
	if start < mid {
		if !book(root.LeftChild, start, end) {
			bookAns = false
		}
	}
	// 如果和右区间有交集，则递归判断一下右区间
	if end > mid {
		if !book(root.RightChild, start, end) {
			bookAns = false
		}
	}
	return bookAns
}

// 下推状态
func pushDownStatus(root *SegmentTreeNode, start int, end int) {
	// 到了不可再分则直接返回
	if root.Left == root.Right-1 {
		return
	}
	// 无关的区间直接返回
	if withoutBookRange(root, start, end) {
		return
	}
	// 如果节点区间完全在预定区间内
	if withInBookRange(root, start, end) {
		root.BookedStatus = 1 // 完全被预定
		return
	}
	// 预定区间存在交集
	var mid = root.Left + (root.Right-root.Left)/2
	if start < mid {
		pushDownStatus(root.LeftChild, start, end)
	}
	if end > mid {
		pushDownStatus(root.RightChild, start, end)
	}
	// 根据左右子树的预定状态更新当前节点的预定状态
	if root.LeftChild.BookedStatus == 0 && root.RightChild.BookedStatus == 0 {
		root.BookedStatus = 0
	} else if root.LeftChild.BookedStatus == 1 && root.RightChild.BookedStatus == 1 {
		root.BookedStatus = 1
	} else {
		root.BookedStatus = 2
	}
	return
}

// 节点区间与预定区间都是左闭右开区间
func withInBookRange(node *SegmentTreeNode, start int, end int) bool {
	if node.Left >= start && node.Right < end {
		return true
	}
	return false
}

// 节点区间与预定区间都是左闭右开区间
func withoutBookRange(node *SegmentTreeNode, start int, end int) bool {
	if node.Left >= end || node.Right <= start {
		return true
	}
	return false
}

func constructChild(child *SegmentTreeNode, left, right int) *SegmentTreeNode {
	if child != nil {
		return child
	}
	return &SegmentTreeNode{
		Left:  left,
		Right: right,
	}
}
