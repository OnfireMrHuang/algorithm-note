class MedianFinder {
	private PriorityQueue<Integer> leftHeap;
	private PriorityQueue<Integer> rightHeap;

	public MedianFinder() {
		// 小顶堆
		rightHeap = new PriorityQueue<>();
		// 大顶堆
		leftHeap = new PriorityQueue<>((a, b) -> {
			return b - a;
		});
	}

	public double findMedian() {
		// 如果元素不一样多，多的那个堆的堆顶元素就是中位数
		if (leftHeap.size() != rightHeap.size()) {
			return leftHeap.peek();
		}
		// 如果元素一样多，两个堆堆顶元素的平均数是中位数
		return (leftHeap.peek() + rightHeap.peek()) / 2.0;
	}

	public void addNum(int num) {
		if (leftHeap.size() == rightHeap.size()) {
			rightHeap.offer(num);
			leftHeap.offer(rightHeap.poll());
		} else {
			leftHeap.offer(num);
			rightHeap.offer(leftHeap.poll());
		}
	}
}