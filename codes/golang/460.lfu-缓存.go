

type LFUCache struct {
	cache          map[int]int
	keyFreqMap     map[int]int
	freqKeyListMap map[int][]int
	capacity       int
	minFreq        int
}

func Constructor(capacity int) LFUCache {
	return LFUCache{
		cache:          make(map[int]int),
		keyFreqMap:     make(map[int]int),
		freqKeyListMap: make(map[int][]int),
		capacity:       capacity,
		minFreq:        0,
	}
}

func (this *LFUCache) Get(key int) int {
	value, ok := this.cache[key]
	if !ok {
		return -1
	}
	this.increaseFreq(key)
	return value
}

func (this *LFUCache) Put(key int, value int) {
	if _, ok := this.cache[key]; ok {
		this.cache[key] = value
		this.increaseFreq(key)
		return
	}
	if len(this.cache) >= this.capacity {
		this.removeMinFreqKey()
	}
	this.cache[key] = value
	this.keyFreqMap[key] = 1
	this.freqKeyListMap[1] = append(this.freqKeyListMap[1], key)
	this.minFreq = 1
}

func (this *LFUCache) increaseFreq(key int) {
	freq := this.keyFreqMap[key]
	this.keyFreqMap[key] = freq + 1
	keyList := this.freqKeyListMap[freq]
	for i, k := range keyList {
		if k == key {
			keyList = append(keyList[:i], keyList[i+1:]...)
			break
		}
	}
	this.freqKeyListMap[freq] = keyList
	if len(keyList) == 0 {
		delete(this.freqKeyListMap, freq)
		if freq == this.minFreq {
			this.minFreq++
		}
	}
	this.freqKeyListMap[freq+1] = append(this.freqKeyListMap[freq+1], key)
}

func (this *LFUCache) removeMinFreqKey() {
	keyList := this.freqKeyListMap[this.minFreq]
	delKey := keyList[0]
	this.freqKeyListMap[this.minFreq] = keyList[1:]
	if len(this.freqKeyListMap[this.minFreq]) == 0 {
		delete(this.freqKeyListMap, this.minFreq)
	}
	delete(this.keyFreqMap, delKey)
	delete(this.cache, delKey)
}

