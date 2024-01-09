

type LRUEntry struct {
	key   int
	value int
	prev  *LRUEntry
	next  *LRUEntry
}

func NewLRUEntry(key, value int) *LRUEntry {
	return &LRUEntry{
		key:   key,
		value: value,
	}
}

type LRUCache struct {
	capacity int
	head     *LRUEntry
	tail     *LRUEntry
	m        map[int]*LRUEntry
}

func Constructor(capacity int) LRUCache {
	head := NewLRUEntry(-1, -1)
	tail := NewLRUEntry(-1, -1)
	head.next = tail
	tail.prev = head
	return LRUCache{
		capacity: capacity,
		head:     head,
		tail:     tail,
		m:        make(map[int]*LRUEntry),
	}
}

func (this *LRUCache) Get(key int) int {
	if entry, ok := this.m[key]; ok {
		this.detach(entry)
		this.attach(entry)
		return entry.value
	}
	return -1
}

func (this *LRUCache) Put(key int, value int) {
	if entry, ok := this.m[key]; ok {
		entry.value = value
		this.detach(entry)
		this.attach(entry)
	} else {
		entry = NewLRUEntry(key, value)
		if len(this.m) == this.capacity {
			delete(this.m, this.tail.prev.key)
			this.detach(this.tail.prev)
		}
		this.attach(entry)
		this.m[key] = entry
	}
}

func (this *LRUCache) detach(entry *LRUEntry) {
	entry.prev.next = entry.next
	entry.next.prev = entry.prev
}

func (this *LRUCache) attach(entry *LRUEntry) {
	entry.next = this.head.next
	entry.prev = this.head
	this.head.next.prev = entry
	this.head.next = entry
}
