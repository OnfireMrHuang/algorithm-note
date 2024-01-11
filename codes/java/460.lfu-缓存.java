
import java.util.HashMap;
import java.util.ArrayList;

class LFUCache {
	private HashMap<Integer, Integer> cache;
	private HashMap<Integer, Integer> key_freq_map;
	private HashMap<Integer, ArrayList<Integer>> freq_key_list_map;
	private Integer capacity;
	private Integer min_freq;

	public LFUCache(int capacity) {
		this.capacity = capacity;
		this.cache = new HashMap<>();
		this.key_freq_map = new HashMap<>();
		this.freq_key_list_map = new HashMap<>();
		this.min_freq = 0;
	}

	public int get(int key) {
		Integer value = this.cache.getOrDefault(key, -1);
		if (value != -1) {
			this.increaseFreq(key);
		}
		return value;
	}

	public void put(int key, int value) {
		// 先判断是否存在
		if (this.cache.containsKey(key)) {
			this.cache.put(key, value);
			this.increaseFreq(key);
			return;
		}
		// 判断是否超出容量
		if (this.cache.size() >= this.capacity) {
			this.removeMinFreqKey();
		}
		// 插入新的
		this.cache.put(key, value);
		this.key_freq_map.put(key, 1);
		this.freq_key_list_map.compute(1, (k, v) -> {
			if (v == null) {
				v = new ArrayList<>();
			}
			v.add(key);
			return v;
		});
		this.min_freq = 1;
	}

	private void increaseFreq(int key) {
		Integer freq = this.key_freq_map.get(key);
		this.key_freq_map.put(key, freq + 1);
		ArrayList<Integer> key_list = this.freq_key_list_map.get(freq);
		for (int i = 0; i < key_list.size(); i++) {
			if (key_list.get(i) == key) {
				key_list.remove(i);
				break;
			}
		}
		if (key_list.size() == 0) {
			this.freq_key_list_map.remove(freq);
			if (freq == this.min_freq) {
				this.min_freq++;
			}
		}
		this.freq_key_list_map.compute(freq + 1, (k, v) -> {
			if (v == null) {
				v = new ArrayList<>();
			}
			v.add(key);
			return v;
		});
	}

	private void removeMinFreqKey() {
		ArrayList<Integer> key_list = this.freq_key_list_map.get(this.min_freq);
		Integer del_key = key_list.get(0);
		key_list.remove(0);
		if (key_list.size() == 0) {
			this.freq_key_list_map.remove(this.min_freq);
			// this.min_freq++; // 这里不需要加1，因为当出现淘汰时，一定是来了一个新key, 那么min_freq一定会被置1
		}
		this.key_freq_map.remove(del_key);
		this.cache.remove(del_key);
	}
}

/**
 * Your LFUCache object will be instantiated and called as such:
 * LFUCache obj = new LFUCache(capacity);
 * int param_1 = obj.get(key);
 * obj.put(key,value);
 */