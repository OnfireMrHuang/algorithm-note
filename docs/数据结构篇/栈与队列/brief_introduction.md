# 栈与队列

## 栈

「栈 stack」是一种遵循先入后出的逻辑的线性数据结构。引用hello-algo的一张图解释:

![stack](https://www.hello-algo.com/chapter_stack_and_queue/stack.assets/stack_operations.png)

<br>

## 队列

「队列 queue」是一种遵循先入先出规则的线性数据结构。引用hello-algo的一张图解释:

![queue](https://www.hello-algo.com/chapter_stack_and_queue/queue.assets/queue_operations.png)

<br>

## 两者对比

### 一、 栈和队列都是线性表，都有同样的一组操作 


|方法|解释|操作效率|
|:---|:---|:---|
|`push()`|入栈或入队|O(1)|
|`pop()`|出栈或出队|O(1)|
|`peek()`|查看栈顶/队首元素|O(1)|
|`isEmpty()`|判断栈或队列是否为空|O(1)|
|`size()`|获取栈或队列的大小|O(1)|

<br>

### 二、栈和队列底层都依赖数组或链表来实现

举例python说明

**栈的链表实现**:

```python
class LinkedListStack:
    """基于链表实现的栈"""

    def __init__(self):
        """构造方法"""
        self._peek: ListNode | None = None
        self._size: int = 0

    def size(self) -> int:
        """获取栈的长度"""
        return self._size

    def is_empty(self) -> bool:
        """判断栈是否为空"""
        return not self._peek

    def push(self, val: int):
        """入栈"""
        node = ListNode(val)
        node.next = self._peek
        self._peek = node
        self._size += 1

    def pop(self) -> int:
        """出栈"""
        num = self.peek()
        self._peek = self._peek.next
        self._size -= 1
        return num

    def peek(self) -> int:
        """访问栈顶元素"""
        if self.is_empty():
            raise IndexError("栈为空")
        return self._peek.val

    def to_list(self) -> list[int]:
        """转化为列表用于打印"""
        arr = []
        node = self._peek
        while node:
            arr.append(node.val)
            node = node.next
        arr.reverse()
        return arr
```

**栈的数组实现**:

```python
class ArrayStack:
    """基于数组实现的栈"""

    def __init__(self):
        """构造方法"""
        self._stack: list[int] = []

    def size(self) -> int:
        """获取栈的长度"""
        return len(self._stack)

    def is_empty(self) -> bool:
        """判断栈是否为空"""
        return self._stack == []

    def push(self, item: int):
        """入栈"""
        self._stack.append(item)

    def pop(self) -> int:
        """出栈"""
        if self.is_empty():
            raise IndexError("栈为空")
        return self._stack.pop()

    def peek(self) -> int:
        """访问栈顶元素"""
        if self.is_empty():
            raise IndexError("栈为空")
        return self._stack[-1]

    def to_list(self) -> list[int]:
        """返回列表用于打印"""
        return self._stack

```

**队列的链表实现**:

```python
class LinkedListQueue:
    """基于链表实现的队列"""

    def __init__(self):
        """构造方法"""
        self._front: ListNode | None = None  # 头节点 front
        self._rear: ListNode | None = None  # 尾节点 rear
        self._size: int = 0

    def size(self) -> int:
        """获取队列的长度"""
        return self._size

    def is_empty(self) -> bool:
        """判断队列是否为空"""
        return not self._front

    def push(self, num: int):
        """入队"""
        # 尾节点后添加 num
        node = ListNode(num)
        # 如果队列为空，则令头、尾节点都指向该节点
        if self._front is None:
            self._front = node
            self._rear = node
        # 如果队列不为空，则将该节点添加到尾节点后
        else:
            self._rear.next = node
            self._rear = node
        self._size += 1

    def pop(self) -> int:
        """出队"""
        num = self.peek()
        # 删除头节点
        self._front = self._front.next
        self._size -= 1
        return num

    def peek(self) -> int:
        """访问队首元素"""
        if self.is_empty():
            raise IndexError("队列为空")
        return self._front.val

    def to_list(self) -> list[int]:
        """转化为列表用于打印"""
        queue = []
        temp = self._front
        while temp:
            queue.append(temp.val)
            temp = temp.next
        return queue

```

**队列的数组实现**:

基于数组实现队列，当需要出队一个元素时，往往需要将数组后面的所有元素往前移动一位，这样的操作效率是`O(n)`, 因此在实现中我们往往通过环形数组的形式来实现，即通过`队首下标 + 元素个数 % 队列大小`来计算插入队列的下标位置，通过该方法将操作效率提升到`O(1)`。

```python
class ArrayQueue:
    """基于环形数组实现的队列"""

    def __init__(self, size: int):
        """构造方法"""
        self._nums: list[int] = [0] * size  # 用于存储队列元素的数组
        self._front: int = 0  # 队首指针，指向队首元素
        self._size: int = 0  # 队列长度

    def capacity(self) -> int:
        """获取队列的容量"""
        return len(self._nums)

    def size(self) -> int:
        """获取队列的长度"""
        return self._size

    def is_empty(self) -> bool:
        """判断队列是否为空"""
        return self._size == 0

    def push(self, num: int):
        """入队"""
        if self._size == self.capacity():
            raise IndexError("队列已满")
        # 计算尾指针，指向队尾索引 + 1
        # 通过取余操作，实现 rear 越过数组尾部后回到头部
        rear: int = (self._front + self._size) % self.capacity()
        # 将 num 添加至队尾
        self._nums[rear] = num
        self._size += 1

    def pop(self) -> int:
        """出队"""
        num: int = self.peek()
        # 队首指针向后移动一位，若越过尾部则返回到数组头部
        self._front = (self._front + 1) % self.capacity()
        self._size -= 1
        return num

    def peek(self) -> int:
        """访问队首元素"""
        if self.is_empty():
            raise IndexError("队列为空")
        return self._nums[self._front]

    def to_list(self) -> list[int]:
        """返回列表用于打印"""
        res = [0] * self.size()
        j: int = self._front
        for i in range(self.size()):
            res[i] = self._nums[(j % self.capacity())]
            j += 1
        return res

```

**两种实现方式对比**:

||空间利用率|扩容|
|:---|:---|:---|
|基于链表实现|栈或队列的实际大小是根据元素个数决定的|因为链表的非连续性，扩展栈大小时非常灵活|
|基于数组实现|栈或队列的大小需要预先分配好，会出现利用空间小于分配空间的情况|因为数组的连续性，扩展栈是可能需要重新分配数组并进行数据拷贝，操作昂贵|
