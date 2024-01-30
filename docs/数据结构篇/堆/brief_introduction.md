# 堆

## 堆的定义

「堆 heap」是一种满足特定条件的完全二叉树，主要可分为两种类型。

- 「大顶堆 max heap」：任意节点的值其子节点的值。
- 「小顶堆 min heap」：任意节点的值其子节点的值。

![堆](https://www.hello-algo.com/chapter_heap/heap.assets/min_heap_and_max_heap.png)

以小根堆为例，常见的操作有: 插入一个数、查询堆顶最小值、删除堆顶最小值、获取堆的大小、合并两个堆等等。

## 堆的实现

### 堆的存储

因为堆是一种完全二叉树，所以可以使用数组来实现，这样可以节省空间。其中下标为 `i` 的节点的左右子节点的下标分别为 `2*i+1` 和 `2*i+2`。当索引越界时，表示空节点或节点不存在.图示如下:

![堆的存储](https://www.hello-algo.com/chapter_heap/heap.assets/representation_of_heap.png)

当要查询堆顶元素时，直接返回数组的第一个元素即可。时间复杂度为 `O(1)`。

### 插入一个数

插入一个数时，一般是直接将数值添加到堆底(也就是直接追加到数组末尾)。但是添加之后，末尾的数值和前面的值的顺序就可能不满足堆的性质了，因此需要重新平衡，需要从堆底元素不断`向上调整`，直到满足堆的性质为止，这个过程称为「堆化 heapify」。时间复杂度为 `O(logn)`。

![插入一个数](https://oi-wiki.org/ds/images/binary_heap_insert.svg)

### 删除堆顶元素

需要注意的是，删除堆顶元素不能直接删除数组中的第一个元素，这样节点索引会发生变化，同时操作的时间复杂度也会变为 `O(n)`。因此，一般的做法是将堆顶元素和堆底元素交换，然后删除堆底元素，最后再从堆顶元素开始`向下调整`，直到满足堆的性质为止, 这个过程也称为「堆化 heapify」，只是方向不同。时间复杂度为 `O(logn)`。

### 代码实现

**节点访问:**

```python
def left(self, i: int) -> int:
    """获取左子节点的索引"""
    return 2 * i + 1

def right(self, i: int) -> int:
    """获取右子节点的索引"""
    return 2 * i + 2

def parent(self, i: int) -> int:
    """获取父节点的索引"""
    return (i - 1) // 2  # 向下整除
```

**向上调整:**

```python
def push(self, val: int):
    """元素入堆"""
    # 添加节点
    self.max_heap.append(val)
    # 从底至顶堆化
    self.sift_up(self.size() - 1)

def sift_up(self, i: int):
    """从节点 i 开始，从底至顶堆化"""
    while True:
        # 获取节点 i 的父节点
        p = self.parent(i)
        # 当“越过根节点”或“节点无须修复”时，结束堆化
        if p < 0 or self.max_heap[i] <= self.max_heap[p]:
            break
        # 交换两节点
        self.swap(i, p)
        # 循环向上堆化
        i = p
```

**向下调整:**

```python
def pop(self) -> int:
    """元素出堆"""
    # 判空处理
    if self.is_empty():
        raise IndexError("堆为空")
    # 交换根节点与最右叶节点（交换首元素与尾元素）
    self.swap(0, self.size() - 1)
    # 删除节点
    val = self.max_heap.pop()
    # 从顶至底堆化
    self.sift_down(0)
    # 返回堆顶元素
    return val

def sift_down(self, i: int):
    """从节点 i 开始，从顶至底堆化"""
    while True:
        # 判断节点 i, l, r 中值最大的节点，记为 ma
        l, r, ma = self.left(i), self.right(i), i
        if l < self.size() and self.max_heap[l] > self.max_heap[ma]:
            ma = l
        if r < self.size() and self.max_heap[r] > self.max_heap[ma]:
            ma = r
        # 若节点 i 最大或索引 l, r 越界，则无须继续堆化，跳出
        if ma == i:
            break
        # 交换两节点
        self.swap(i, ma)
        # 循环向下堆化
        i = ma
```

## 堆的应用

- 优先队列: 堆经常作为优先级队列的底层数据结构，优先级队列是一种特殊的队列，它的出队顺序和入队顺序无关，而是和优先级相关(比如按大小优先级)。优先级高的元素先出队，优先级相同的元素按照入队顺序出队。
- `top-k`问题: 给定一个无序数组，求出其中最大的 `k` 个数。常见场景比如排行榜、热搜榜等等。
- `中位数`问题: 给定一个无序数组，求出其中的中位数。该场景下通常使用两个堆来实现，一个大根堆，一个小根堆，大根堆存储数组中较小的一半元素，小根堆存储数组中较大的一半元素。这样中位数就是两个堆顶元素的平均值。
