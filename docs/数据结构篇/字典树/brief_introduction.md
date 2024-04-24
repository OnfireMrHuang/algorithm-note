# 字典树

## 定义

![字典树定义](https://oi-wiki.org/string/images/trie1.png)

字典树也叫`Trie`树，它是一种树形结构，用于专门处理字符串匹配的数据机构，解决在一组字符串集合中快速查找某个字符串的问题。你可能会问，这个我们用哈希表不也可以实现吗？是的，哈希表也可以实现，但是相比哈希表每个字符串都存储一个哈希桶，字典树可以共享相同前缀的字符串，节省空间。而对比普通的文本匹配，字典树又可以通过公共前缀来降低查询时间的开销，以此提升效率。

字典树有三个基本性质:

1. 根节点不包含字符，除根节点外每一个节点都只包含一个字符
2. 从根节点到某一节点，路径上经过的字符连接起来，为该节点对应的字符串
3. 每个节点的所有子节点包含的字符都不相同

## 实现

```python
class Node:                                     # 字符节点
    def __init__(self):                         # 初始化字符节点
        self.children = dict()                  # 初始化子节点
        self.isEnd = False                      # isEnd 用于标记单词结束
        
        
class Trie:                                     # 字典树
    
    # 初始化字典树
    def __init__(self):                         # 初始化字典树
        self.root = Node()                      # 初始化根节点（根节点不保存字符）

    # 向字典树中插入一个单词
    def insert(self, word: str) -> None:
        cur = self.root
        for ch in word:                         # 遍历单词中的字符
            if ch not in cur.children:          # 如果当前节点的子节点中，不存在键为 ch 的节点
                cur.children[ch] = Node()       # 建立一个节点，并将其保存到当前节点的子节点
            cur = cur.children[ch]              # 令当前节点指向新建立的节点，继续处理下一个字符
        cur.isEnd = True                        # 单词处理完成时，将当前节点标记为单词结束

    # 查找字典树中是否存在一个单词
    def search(self, word: str) -> bool:
        cur = self.root
        for ch in word:                         # 遍历单词中的字符
            if ch not in cur.children:          # 如果当前节点的子节点中，不存在键为 ch 的节点
                return False                    # 直接返回 False
            cur = cur.children[ch]              # 令当前节点指向新建立的节点，然后继续查找下一个字符

        return cur is not None and cur.isEnd    # 判断当前节点是否为空，并且是否有单词结束标记

    # 查找字典树中是否存在一个前缀
    def startsWith(self, prefix: str) -> bool:
        cur = self.root
        for ch in prefix:                       # 遍历前缀中的字符
            if ch not in cur.children:          # 如果当前节点的子节点中，不存在键为 ch 的节点
                return False                    # 直接返回 False
            cur = cur.children[ch]              # 令当前节点指向新建立的节点，然后继续查找下一个字符
        return cur is not None                  # 判断当前节点是否为空，不为空则查找成功

```

## 字典树的应用

字典树的应用大致分为以下几种:

- 字符串检索: 通过事先将已知的一些字符串(字典)的信息存储到字典树中，后续通过字典树来检索某字符串是否存在于字典中、出现的频率
- 前缀统计: 统计一个串所有前缀单词的个数，只需要统计从根节点到叶子节点路径上单词出现的个数，也可以判断一个单词是否为另外一个单词的前缀
- 最长公共前缀问题: 利用字典树求解多个字符串的最长公共前缀。将大量字符串都存储到一颗字典树上时，可以快速得到某些字符串的公共前缀。对于所有字符串都建立字典树，两个串的最长公共前缀的长度就是它们所在节点最近公共祖先的长度。
- 字符串排序: 利用字典树进行串排序。给定多个互不相同的仅由一个单词构成的英文名，将它们按字典从小到大输出。采用数组方式创建字典树，字典树中每个节点的所有子节点都是按照其字母大小排序的，然后对字典树进行先序遍历，输出的相应字符串就是按字典序排序的结果。

## 参考资料

> 算法通关手册 https://algo.itcharge.cn/06.String/03.String-Multi-Pattern-Matching/01.Trie/#_5-%E5%AD%97%E5%85%B8%E6%A0%91%E7%9A%84%E5%BA%94%E7%94%A8