import java.util.PriorityQueue;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.ArrayList;
import java.util.Collections;

class Item implements Comparable<Item> {
	String word;
	int count;

	public Item(String word, int count) {
		this.word = word;
		this.count = count;
	}

	@Override
	public int compareTo(Item o) {
		if (this.count == o.count) {
			// 按字典顺序
			return o.word.compareTo(this.word);
		}
		return this.count - o.count;
	}
}

class Solution {
	public List<String> topKFrequent(String[] words, int k) {
		// 构造单词-频率
		HashMap<String, Integer> map = new HashMap<>();
		for (String word : words) {
			map.put(word, map.getOrDefault(word, 0) + 1);
		}

		// 遍历映射并插入元素到大根堆
		PriorityQueue<Item> queue = new PriorityQueue<>();
		for (Map.Entry<String, Integer> item : map.entrySet()) {
			queue.offer(new Item(item.getKey(), item.getValue()));
			if (queue.size() > k) {
				queue.poll();
			}
		}

		List<String> result = new ArrayList<>();
		for (int i = 0; i < k; i++) {
			result.add(queue.poll().word);
		}
		Collections.reverse(result);
		return result;
	}
}