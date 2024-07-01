package my_calendar_i

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

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
	return book(this.root, start, end)
}

func book(root *SegmentTreeNode, start int, end int) bool {
	// 如果已经切到叶子节点
	if root.Left == root.Right {
		return root.Left >= start && root.Right < end
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
			root.BookedStatus = 1 // 设置为全部被预定
			ans = true
		default:
			// 存在冲突，返回预定失败
			ans = false
		}
		return ans
	}
	// 预定区间存在交集
	var mid = root.Left + (root.Right-root.Left)/2
	root.LeftChild = constructChild(root.LeftChild, root.Left, mid)    // 构造左子树
	root.RightChild = constructChild(root.RightChild, mid, root.Right) // 构造右子树
	var bookAns = true
	if start < mid {
		if !book(root.LeftChild, start, end) {
			bookAns = false
		}
	}
	if end >= mid {
		if !book(root.RightChild, start, end) {
			bookAns = false
		}
	}
	// 根据左右子树的预定状态更新当前节点的预定状态
	if root.LeftChild.BookedStatus == 0 && root.RightChild.BookedStatus == 0 {
		root.BookedStatus = 0
	} else if root.LeftChild.BookedStatus == 1 && root.RightChild.BookedStatus == 1 {
		root.BookedStatus = 1
	} else {
		root.BookedStatus = 2
	}
	return bookAns
}

func withInBookRange(node *SegmentTreeNode, start int, end int) bool {
	if node.Left >= start && node.Right < end {
		return true
	}
	return false
}

func withoutBookRange(node *SegmentTreeNode, start int, end int) bool {
	if node.Left >= end || node.Right < start {
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

func TestMyCalendar(t *testing.T) {
	var obj = Constructor()
	var ans bool
	ans = obj.Book(10, 20)
	assert.Equal(t, true, ans)
	ans = obj.Book(15, 25)
	assert.Equal(t, false, ans)
	ans = obj.Book(20, 30)
	assert.Equal(t, true, ans)
}
