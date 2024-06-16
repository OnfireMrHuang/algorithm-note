// Go语言中没有内置的BTree数据结构，这里需要手写一个线段树定义

type RangeNode struct {
	Start: int,
	End: int
	Left: *RangeTree,
	Right: *RangeTree,
}

func NewRangeNode(start,end int) *RangeNode {
	return &RangeNode{
		Start: start,
		End: end,
	}
}

func 


type MyCalendar struct {
	root *RangeTree
}

func Constructor() MyCalendar {
	return MyCalendar{}
}

func (this *MyCalendar) Book(start int, end int) bool {
	if this.root == nil {
		this.root = NewRangeNode(start, end)
		return true
	}
	// 首先与根节点对比，如果不在根节点范围内


}

func (this *MyCalendar) book(node *RangeNode,start int, end int) {

}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * obj := Constructor();
 * param_1 := obj.Book(start,end);
 */