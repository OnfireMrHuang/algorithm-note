// 用优先级队列解决这道题
class Solution {
	public int[] topKFrequent(int[] nums, int k) {
		// nums 中的元素 -> 该元素出现的频率
		HashMap<Integer, Integer> valToFreq = new HashMap<>();
		for (int v : nums) {
			valToFreq.put(v, valToFreq.getOrDefault(v, 0) + 1);
		}

		PriorityQueue<Map.Entry<Integer, Integer>> pq = new PriorityQueue<>((entry1, entry2) -> {
			// 队列按照键值对中的值（元素出现频率）从小到大排序
			return entry1.getValue().compareTo(entry2.getValue());
		});

		for (Map.Entry<Integer, Integer> entry : valToFreq.entrySet()) {
			pq.offer(entry);
			if (pq.size() > k) {
				// 弹出最小元素，维护队列内是 k 个频率最大的元素
				pq.poll();
			}
		}

		int[] res = new int[k];
		for (int i = k - 1; i >= 0; i--) {
			// res 数组中存储前 k 个最大元素
			res[i] = pq.poll().getKey();
		}

		return res;
	}
}

// 用计数排序的方法解决这道题
class Solution2 {
	public int[] topKFrequent(int[] nums, int k) {
		// nums 中的元素 -> 该元素出现的频率
		HashMap<Integer, Integer> valToFreq = new HashMap<>();
		for (int v : nums) {
			valToFreq.put(v, valToFreq.getOrDefault(v, 0) + 1);
		}

		// 频率 -> 这个频率有哪些元素
		ArrayList<Integer>[] freqToVals = new ArrayList[nums.length + 1];
		for (int val : valToFreq.keySet()) {
			int freq = valToFreq.get(val);
			if (freqToVals[freq] == null) {
				freqToVals[freq] = new ArrayList<>();
			}
			freqToVals[freq].add(val);
		}

		int[] res = new int[k];
		int p = 0;
		// freqToVals 从后往前存储着出现最多的元素
		for (int i = freqToVals.length - 1; i > 0; i--) {
			ArrayList<Integer> valList = freqToVals[i];
			if (valList == null)
				continue;
			for (int j = 0; j < valList.size(); j++) {
				// 将出现次数最多的 k 个元素装入 res
				res[p] = valList.get(j);
				p++;
				if (p == k) {
					return res;
				}
			}
		}

		return null;
	}
}