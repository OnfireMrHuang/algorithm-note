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

## 438. 找到字符串中所有字母异位词

> [题目描述](https://leetcode-cn.com/problems/find-all-anagrams-in-a-string/)

</br>

## 1044. 最长重复子串

> [题目描述](https://leetcode-cn.com/problems/longest-duplicate-substring/)

</br>

## 146. LRU缓存机制

> [题目描述](https://leetcode-cn.com/problems/lru-cache/)

</br>

## 460. LFU缓存

> [题目描述](https://leetcode-cn.com/problems/lfu-cache/)

</br>
