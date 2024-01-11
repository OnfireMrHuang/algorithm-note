


class LFUCache:

    def __init__(self, capacity: int):
        self.capacity = capacity
        self.cache = {}
        self.key_freq_map = {}
        self.freq_key_list_map = {}
        self.min_freq = 0


    def get(self, key: int) -> int:
        value = self.cache.get(key, -1)
        if value != -1:
            self.increaseFreq(key)
        return value


    def put(self, key: int, value: int) -> None:
        if key in self.cache:
            self.cache[key] = value
            self.increaseFreq(key)
            return
        if len(self.cache) >= self.capacity:
            self.removeMinFreqKey()
        self.cache[key] = value
        self.key_freq_map[key] = 1
        self.freq_key_list_map[1] = self.freq_key_list_map.get(1, []) + [key]
        self.min_freq = 1

    def increaseFreq(self, key):
        freq = self.key_freq_map[key]
        self.key_freq_map[key] = freq + 1
        key_list = self.freq_key_list_map[freq]
        key_list.remove(key)
        if len(key_list) == 0:
            self.freq_key_list_map.pop(freq)
            if freq == self.min_freq:
                self.min_freq += 1
        self.freq_key_list_map[freq + 1] = self.freq_key_list_map.get(freq + 1, []) + [key]

    def removeMinFreqKey(self):
        key_list = self.freq_key_list_map[self.min_freq]
        del_key = key_list[0]
        key_list.remove(del_key)
        if len(key_list) == 0:
            self.freq_key_list_map.pop(self.min_freq)
        self.key_freq_map.pop(del_key)
        self.cache.pop(del_key)



# Your LFUCache object will be instantiated and called as such:
# obj = LFUCache(capacity)
# param_1 = obj.get(key)
# obj.put(key,value)
        