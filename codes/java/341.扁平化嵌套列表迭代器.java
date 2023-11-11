public class NestedIterator implements Iterator<Integer> {
	private LinkedList<NestedInteger> list;

	public NestedIterator(List<NestedInteger> nestedList) {
		// 不直接用 nestedList 的引用，是因为不能确定它的底层实现
		// 必须保证是 LinkedList，否则下面的 addFirst 会很低效
		list = new LinkedList<>(nestedList);
	}

	public Integer next() {
		// hasNext 方法保证了第一个元素一定是整数类型
		return list.remove(0).getInteger();
	}

	public boolean hasNext() {
		// 循环拆分列表元素，直到列表第一个元素是整数类型
		while (!list.isEmpty() && !list.get(0).isInteger()) {
			// 当列表开头第一个元素是列表类型时，进入循环
			List<NestedInteger> first = list.remove(0).getList();
			// 将第一个列表打平并按顺序添加到开头
			for (int i = first.size() - 1; i >= 0; i--) {
				list.addFirst(first.get(i));
			}
		}
		return !list.isEmpty();
	}
}