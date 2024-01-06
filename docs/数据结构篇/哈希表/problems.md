# 题解

## 217. 存在重复元素

> [题目描述](https://leetcode-cn.com/problems/contains-duplicate/)

**题目解法**: 该题目可以使用哈希表轻松解决，对于每一个元素，如果哈希表中不存在该元素，则将该元素加入哈希表中，否则返回`true`。

[rust版本](../../../codes/rust/217.存在重复元素.rs) |
[java版本](../../../codes/java/217.存在重复元素.java) |
[golang版本](../../../codes/golang/217.存在重复元素.go) |
[python](../../../codes/python/217.存在重复元素.py)

</br>

## 219. 存在重复元素 II

> [题目描述](https://leetcode-cn.com/problems/contains-duplicate-ii/)

**题目解法**: 该题目同样可以使用哈希表解决，对于每一个元素，如果哈希表中不存在该元素，则将该元素加入哈希表中，否则判断该元素与哈希表中该元素的下标差是否小于等于`k`，如果是则返回`true`，否则继续遍历。

[rust版本](../../../codes/rust/219.存在重复元素II.rs) |
[java版本](../../../codes/java/219.存在重复元素II.java) |
[golang版本](../../../codes/golang/219.存在重复元素II.go) |
[python](../../../codes/python/219.存在重复元素II.py)

</br>

## 1044. 最长重复子串

> [题目描述](https://leetcode-cn.com/problems/longest-duplicate-substring/)

**题目解法**:

一、暴力解决:

对于查找字符串中的最长重复子串，如果我们不考虑性能，那么我们可以针对所有子串进行判断(从该子串的开始索引到字符串的末尾，对比每一个等长的不同子串)，如果该子串在字符串中出现了多次，那么就对比其长度是不是最长，如果是则记录开始结束位置，知道判断完所有子串。

其中: 对于查找所有子串，其时间复杂度为`O(n^2)`，对于每一个子串的判断，因为是逐个子串逐个字符对比，所以时间复杂度为`O(n^2)`，所以总的时间复杂度为`O(n^4)`。
示例代码如下:

```python
start, end = 0, 0
max_len = 0
for i in range(len(s)):
 for j in range(i + 1, len(s)):
  # 子串s[i:j]
  step = 1
  is_duplicate = False
  # O(n)的时间复杂度判断子串是否重复
  for k in range(j + 1, len(s)):
   # 判断s[i:j]与s[i+step:k]是否相等
   if s[i:j] == s[i+step:k]:
	is_duplicate = True
	break
   step += 1
  # 如果子串重复，判断其长度是否最长
  if is_duplicate and j - i > max_len:
   start, end = i, j
   max_len = j - i
# 最后返回最长重复子串
return s[start:end]
```

二、二分查找:

在版本一中，我们对每一个子串都进行了判断，对此我们再进一步思考一下，对于一个长度为`a_len`的子串`a`:

- 如果它存在重复子串，那么长度比`a_len`更小的子串就没必要再搜索判断了，因为他们肯定不存在最长重复子串了
- 如果它不存在重复子串，那么比`a_len`更大的子串，也没有必要再继续搜索判断了，因为如果存在更长的重复子串，那么`a`肯定存在它的重复子串

所以我们优化为使用二分查找的思路来查找子串，同时，对比子串的判断，我们是逐个字符对比，这里我们可以优化为使用Rabin-Karp算法来对比子串，这样可以将判断子串的时间复杂度降低到`O(1)`，所以总的时间复杂度为`O(n^2logn)`。

[rust版本](../../../codes/rust/1044.最长重复子串.rs) |
[java版本](../../../codes/java/1044.最长重复子串.java) |
[golang版本](../../../codes/golang/1044.最长重复子串.go) |
[python](../../../codes/python/1044.最长重复子串.py)

</br>

## 146. LRU缓存机制

> [题目描述](https://leetcode-cn.com/problems/lru-cache/)

</br>

## 460. LFU缓存

> [题目描述](https://leetcode-cn.com/problems/lfu-cache/)

</br>
