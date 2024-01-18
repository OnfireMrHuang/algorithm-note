import java.util.PriorityQueue;
import java.util.HashMap;

// 定义一个双堆数据结构，其中维护一个大根堆和一个小根堆
class DualHeap {
	// 大根堆放置较小的一半元素
	private PriorityQueue<Integer> small;
	// 小根堆放置较大的一半元素
	private PriorityQueue<Integer> large;
	// 堆大小
	private int k;
	// 大根堆和小根堆元素个数
	private int smallSize, largeSize;
	// 延迟删除的元素，key为元素，value为需要删除的次数
	private HashMap<Integer, Integer> delayed;

	public DualHeap(int k) {
		this.small = new PriorityQueue<Integer>(new Comparator<Integer>() {
			public int compare(Integer num1, Integer num2) {
				return num2.compareTo(num1);
			}
		});
		this.large = new PriorityQueue<Integer>(new Comparator<Integer>() {
			public int compare(Integer num1, Integer num2) {
				return num1.compareTo(num2);
			}
		});
		this.k = k;
		this.smallSize = 0;
		this.largeSize = 0;
		this.delayed = new HashMap<Integer, Integer>();
	}

	// 获取中位数
	public double getMedian() {
		return (k & 1) == 1 ? small.peek() : ((double) small.peek() + large.peek()) / 2;
	}

	// 插入数据
	public void insert(int num) {
		if (small.isEmpty() || num <= small.peek()) {
			small.offer(num);
			++smallSize;
		} else {
			large.offer(num);
			++largeSize;
		}
		makeBalance();
	}

	// 删除数据
	public void erase(int num) {
		delayed.put(num, delayed.getOrDefault(num, 0) + 1);
		if (num <= small.peek()) {
			--smallSize;
			if (num == small.peek()) {
				prune(small);
			}
		} else {
			--largeSize;
			if (num == large.peek()) {
				prune(large);
			}
		}
		makeBalance();
	}

	// 将堆顶元素移除
	private void prune(PriorityQueue<Integer> heap) {
		while (!heap.isEmpty()) {
			int num = heap.peek();
			if (delayed.containsKey(num)) {
				delayed.put(num, delayed.get(num) - 1);
				if (delayed.get(num) == 0) {
					delayed.remove(num);
				}
				heap.poll();
			} else {
				break;
			}
		}
	}

	// 调整两个堆的大小
	private void makeBalance() {
		if (smallSize > largeSize + 1) {
			large.offer(small.poll());
			--smallSize;
			++largeSize;
			prune(small);
		} else if (smallSize < largeSize) {
			small.offer(large.poll());
			++smallSize;
			--largeSize;
			prune(large);
		}
	}
}

class Solution {
	public double[] medianSlidingWindow(int[] nums, int k) {
		DualHeap dh = new DualHeap(k);
		for (int i = 0; i < k; ++i) {
			dh.insert(nums[i]);
		}
		double[] ans = new double[nums.length - k + 1];
		ans[0] = dh.getMedian();
		for (int i = k; i < nums.length; ++i) {
			dh.insert(nums[i]);
			dh.erase(nums[i - k]);
			ans[i - k + 1] = dh.getMedian();
		}
		return ans;
	}
}