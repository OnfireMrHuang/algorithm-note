
# import java.util.HashMap;

# class LRUEntry {
# 	int key;
# 	int value;
# 	LRUEntry prev;
# 	LRUEntry next;

# 	public LRUEntry(int key, int value) {
# 		this.key = key;
# 		this.value = value;
# 	}
# }

# class LRUCache {
# 	private HashMap<Integer, LRUEntry> map; // key -> entry
# 	private LRUEntry head; // dummy head
# 	private LRUEntry tail; // dummy tail
# 	private int capacity; // capacity

# 	public LRUCache(int capacity) {
# 		this.capacity = capacity;
# 		map = new HashMap<>();
# 		head = new LRUEntry(-1, -1);
# 		tail = new LRUEntry(-1, -1);
# 		head.next = tail;
# 		tail.prev = head;
# 	}

# 	public int get(int key) {
# 		LRUEntry value = this.map.get(key);
# 		if (value == null) {
# 			return -1;
# 		}
# 		detach(value);
# 		attach(value);
# 		return value.value;
# 	}

# 	public void put(int key, int value) {
# 		LRUEntry entry = this.map.get(key);
# 		if (entry != null) {
# 			entry.value = value;
# 			detach(entry);

class LRUEntry:
    def __init__(self, key: int, value: int):
        self.key = key
        self.value = value
        self.prev = None
        self.next = None


class LRUCache:

    def __init__(self, capacity: int):
        self.capacity = capacity
        self.map = {}
        self.head = LRUEntry(-1, -1)
        self.tail = LRUEntry(-1, -1)
        self.head.next = self.tail
        self.tail.prev = self.head

    def attach(self, entry: LRUEntry):
        entry.next = self.head.next
        entry.prev = self.head
        self.head.next.prev = entry
        self.head.next = entry

    def detach(self, entry: LRUEntry):
        entry.prev.next = entry.next
        entry.next.prev = entry.prev

    def get(self, key: int) -> int:
        value = self.map.get(key)
        if value is None:
            return -1
        self.detach(value)
        self.attach(value)
        return value.value


    def put(self, key: int, value: int) -> None:
        entry = self.map.get(key)
        if entry is not None:
            entry.value = value
            self.detach(entry)
            self.attach(entry)
        else:
            entry = LRUEntry(key, value)
            if len(self.map) == self.capacity:
                self.map.pop(self.tail.prev.key)
                self.detach(self.tail.prev)
            self.attach(entry)
            self.map[key] = entry


