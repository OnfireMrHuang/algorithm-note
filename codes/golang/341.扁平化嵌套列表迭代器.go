
type NestedIterator struct {
	list []NestedInteger
}

func Constructor(nestedList []*NestedInteger) *NestedIterator {
	list := make([]NestedInteger, 0)
	for _, nestedInt := range nestedList {
		list = append(list, *nestedInt)
	}
	return &NestedIterator{list}
}

func (this *NestedIterator) Next() int {
	// hasNext 方法保证了第一个元素一定是整数类型
	res := this.list[0].GetInteger()
	this.list = this.list[1:]
	return res
}

func (this *NestedIterator) HasNext() bool {
	// 循环拆分列表元素，直到列表第一个元素是整数类型
	for len(this.list) > 0 && !this.list[0].IsInteger() {
		// 当列表开头第一个元素是列表类型时，进入循环
		first := this.list[0].GetList()
		this.list = this.list[1:]
		// 将第一个列表打平并按顺序添加到开头
		for i := len(first) - 1; i >= 0; i-- {
			this.list = append([]NestedInteger{first[i]}, this.list...)
		}
	}
	return len(this.list) > 0
}