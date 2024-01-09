
import java.util.HashMap;

class LRUEntry {
	int key;
	int value;
	LRUEntry prev;
	LRUEntry next;

	public LRUEntry(int key, int value) {
		this.key = key;
		this.value = value;
	}
}

class LRUCache {
	private HashMap<Integer, LRUEntry> map; // key -> entry
	private LRUEntry head; // dummy head
	private LRUEntry tail; // dummy tail
	private int capacity; // capacity

	public LRUCache(int capacity) {
		this.capacity = capacity;
		map = new HashMap<>();
		head = new LRUEntry(-1, -1);
		tail = new LRUEntry(-1, -1);
		head.next = tail;
		tail.prev = head;
	}

	public int get(int key) {
		LRUEntry value = this.map.get(key);
		if (value == null) {
			return -1;
		}
		detach(value);
		attach(value);
		return value.value;
	}

	public void put(int key, int value) {
		LRUEntry entry = this.map.get(key);
		if (entry != null) {
			entry.value = value;
			detach(entry);
			attach(entry);
		} else {
			entry = new LRUEntry(key, value);
			if (this.map.size() == this.capacity) {
				this.map.remove(tail.prev.key);
				detach(tail.prev);
			}
			attach(entry);
			this.map.put(key, entry);
		}
	}

	public int capacity() {
		return capacity;
	}

	public int size() {
		return map.size();
	}

	// 从链表中删除entry
	void detach(LRUEntry entry) {
		entry.prev.next = entry.next;
		entry.next.prev = entry.prev;
	}

	// 将entry插入到链表头部
	void attach(LRUEntry entry) {
		entry.next = head.next;
		entry.prev = head;
		head.next.prev = entry;
		head.next = entry;
	}
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * LRUCache obj = new LRUCache(capacity);
 * int param_1 = obj.get(key);
 * obj.put(key,value);
 */