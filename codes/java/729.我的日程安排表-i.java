import java.util.TreeSet;

class Range implements Comparable<Range> {
	int start;
	int end;

	public Range(int start, int end) {
		this.start = start;
		this.end = end;
	}

	@Override
	public int compareTo(Range other) {
		if (this.start == other.start) {
			return this.end - other.end;
		}
		return this.start - other.start;
	}

}

class MyCalendar {

	private TreeSet<Range> rangeTree;

	public MyCalendar() {
		rangeTree = new TreeSet<>();
	}

	public boolean book(int start, int end) {
		if (rangeTree.isEmpty()) {
			rangeTree.add(new Range(start, end));
			return true;
		}
		// 往右找到第一个大于等于start的区间
		Range bookRange = new Range(start, end);
		Range nextRange = rangeTree.ceiling(bookRange);
		if (nextRange != null && nextRange.start < end) {
			return false;
		}
		Range prevRange = rangeTree.floor(bookRange);
		if (prevRange != null && prevRange.end > start) {
			return false;
		}
		rangeTree.add(new Range(start, end));
		return true;
	}
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * MyCalendar obj = new MyCalendar();
 * boolean param_1 = obj.book(start,end);
 */