# 线段树

## 线段树的定义

线段树是一种基于分治思想的二叉树，用于统计区间信息，线段树的每一个节点都对应一个区间[left, right], left、right通常是整数。其中叶子节点对应的区间left==right，非叶子节点的左子节点表示区间为[left, (left+right)/2], 右子节点表示的区间为[(left+right)/2 + 1, right].

如下是一个示例的线段树:

```
		   [0, 7]
		   /        \
		[0, 3]	    [4, 7]
	      /    \        /     \
	[0, 1]	[2, 3]	[4, 5]	[6, 7]
    /  \	  /   \	       /   \	    /   \
[0, 0] [1, 1] [2, 2] [3, 3] [4, 4] [5, 5] [6, 6] [7, 7]
```

## 线段树的构造

我们知道二叉树有两种存储结构，一种是【链式】存储结构，一种是【顺序】存储结构，而线段树几乎是满二叉树，所以非常适合使用【顺序】存储结构。其特性如下:

- 根节点的下标索引为0
- 对于任意节点i，其左子节点的索引为2*i+1，右子节点的索引为2*i+2
- 对于非根节点i, 其父节点的索引为(i-1)/2
- 对于[0, N]的区间，理想情况下，其节点数大概为`2 x N - 1`(不断二分分裂，直到单个节点), 通常使用2 x N的数组来存储线段树就足够了； 但是实际情况下, 有些区间元素需要开辟新的一层来存储元素,线段树的深度为`log2(N)`, 最坏的情况下，叶子节点的数量为`2^log2(N)`, 总的节点数量为`2^(log2(N)+1)-1`, 近似为`4 x N`;

根据上面的特性，线段树非常适合采用递归的方式来创建,实现如下:

```python
# 线段树的节点类
class TreeNode:
    def __init__(self, val=0):
        self.left = -1                              # 区间左边界
        self.right = -1                             # 区间右边界
        self.val = val                              # 节点值（区间值）
        self.lazy_tag = None                        # 区间和问题的延迟更新标记
        
        
# 线段树类
class SegmentTree:
    def __init__(self, nums, function):
        self.size = len(nums)
        self.tree = [TreeNode() for _ in range(4 * self.size)]  # 维护 TreeNode 数组
        self.nums = nums                            # 原始数据
        self.function = function                    # function 是一个函数，左右区间的聚合方法
        if self.size > 0:
            self.__build(0, 0, self.size - 1)
            
    # 构建线段树，节点的存储下标为 index，节点的区间为 [left, right]
    def __build(self, index, left, right):
        self.tree[index].left = left
        self.tree[index].right = right
        if left == right:                           # 叶子节点，节点值为对应位置的元素值
            self.tree[index].val = self.nums[left]
            return
    
        mid = left + (right - left) // 2            # 左右节点划分点
        left_index = index * 2 + 1                  # 左子节点的存储下标
        right_index = index * 2 + 2                 # 右子节点的存储下标
        self.__build(left_index, left, mid)         # 递归创建左子树
        self.__build(right_index, mid + 1, right)   # 递归创建右子树
        self.__pushup(index)                        # 向上更新节点的区间值
    
    # 向上更新下标为 index 的节点区间值，节点的区间值等于该节点左右子节点元素值的聚合计算结果
    def __pushup(self, index):
        left_index = index * 2 + 1                  # 左子节点的存储下标
        right_index = index * 2 + 2                 # 右子节点的存储下标
        self.tree[index].val = self.function(self.tree[left_index].val, self.tree[right_index].val)

```

## 线段树的操作

### 单点更新

**修改一个元素的值，比如将第i个元素的值修改为val.**

- 如果是叶子节点，满足 left==right，则更新该节点的值。
- 如果是非叶子节点，则判断应该在左子树中更新，还是应该在右子树中更新。
- 在对应的左子树或右子树中更新节点值。
- 左右子树更新返回之后，向上更新节点的区间值（区间和、区间最大值、区间最小值等），区间值等于该节点左右子节点元素值的聚合计算结果。

```python
    # 单点更新，将 nums[i] 更改为 val
    def update_point(self, i, val):
        self.nums[i] = val
        self.__update_point(i, val, 0, 0, self.size - 1)
        
    # 单点更新，将 nums[i] 更改为 val。节点的存储下标为 index，节点的区间为 [left, right]
    def __update_point(self, i, val, index, left, right):
        if self.tree[index].left == self.tree[index].right:
            self.tree[index].val = val              # 叶子节点，节点值修改为 val
            return
        
        mid = left + (right - left) // 2            # 左右节点划分点
        left_index = index * 2 + 1                  # 左子节点的存储下标
        right_index = index * 2 + 2                 # 右子节点的存储下标
        if i <= mid:                                # 在左子树中更新节点值
            self.__update_point(i, val, left_index, left, mid)
        else:                                       # 在右子树中更新节点值
            self.__update_point(i, val, right_index, mid + 1, right)
        self.__pushup(index)                        # 向上更新节点的区间值
```

### 区间查询

**比如查询一个区间[q_left, q_right]的区间值**

可以采用递归的方式进行区间查询，步骤如下:

1. 如果当前节点的区间[left, right]与查询区间[q_left, q_right]没有交集，则返回一个不影响结果的值，比如0或者-inf。
2. 如果当前节点的区间[left, right]完全在查询区间[q_left, q_right]中，即left > q_left && right < q_right，则返回当前节点的值。
3. 如果当前节点的区间于查询区间构成交集，则:
   1. 判断查询区间[q_left, q_right]是否与节点左子树区间[left, mid]有交集(即q_left < mid)，如果有，则递归查询左子树并保存结果为left_res。
   2. 判断查询区间[q_left, q_right]是否与节点右子树区间[mid+1, right]有交集(即q_right > mid)，如果有，则递归查询右子树并保存结果为right_res。
   3. 最后返回left_res和right_res的聚合结果。

代码如下:

```python
    # 区间查询，查询区间为 [q_left, q_right] 的区间值
    def query_interval(self, q_left, q_right):
        return self.__query_interval(q_left, q_right, 0, 0, self.size - 1)
    
    # 区间查询，在线段树的 [left, right] 区间范围中搜索区间为 [q_left, q_right] 的区间值
    def __query_interval(self, q_left, q_right, index, left, right):
        if left >= q_left and right <= q_right:     # 节点所在区间被 [q_left, q_right] 所覆盖
            return self.tree[index].val             # 直接返回节点值
        if right < q_left or left > q_right:        # 节点所在区间与 [q_left, q_right] 无关
            return 0
    
        self.__pushdown(index)
    
        mid = left + (right - left) // 2            # 左右节点划分点
        left_index = index * 2 + 1                  # 左子节点的存储下标
        right_index = index * 2 + 2                 # 右子节点的存储下标
        res_left = 0                                # 左子树查询结果
        res_right = 0                               # 右子树查询结果
        if q_left <= mid:                           # 在左子树中查询
            res_left = self.__query_interval(q_left, q_right, left_index, left, mid)
        if q_right > mid:                           # 在右子树中查询
            res_right = self.__query_interval(q_left, q_right, right_index, mid + 1, right)
        return self.function(res_left, res_right)   # 返回左右子树元素值的聚合计算结果
```

### 区间更新

**对查询区间[q_left, q_right]进行更新，比如将该区间内所有元素都更新为val**

**1、延迟标记:**

在进行【区间更新】操作中，如果某个节点区间[left, right]被修改区间[q_left, q_right]完全覆盖，那么以该节点为根的整颗子树所有节点的区间值都要发生变化，这样使得一次区间更新操作的时间复杂度增加到O(n).

为了解决这个问题，我们可以引入【延迟标记】的概念，即在节点中维护一个lazy_tag，表示【该节点区间曾经被修改为val,但其子节点区间值尚未更新】。也就是说除了在进行区间更新时，将区间子节点的更新操作延迟到【在后续操作中递归进入子节点时】再执行。这样一来，每次区间更新和区间查询的时间复杂度都是O(logn)。其区间更新步骤如下:

1. 如果区间[q_left, q_right]完全覆盖当前节点区间[left, right]，即q_left <= left && q_right >= right，则更新当前节点的lazy_tag为val，并将当前节点的延迟标记为区间值。
2. 如果区间[q_left, q_right]与当前节点区间[left, right]没有交集，则直接返回。
3. 如果区间[q_left, q_right]与当前节点区间[left, right]有交集，则:
   1. 如果当前节点的lazy_tag不为空，则将当前节点的lazy_tag更新到左右子节点(即向下更新)。
   2. 如果区间[q_left, q_right]与当前节点左子节点区间[left, mid]有交集(即q_left <= mid)，则递归更新左子树。
   3. 如果区间[q_left, q_right]与当前节点右子节点区间[mid+1, right]有交集(即q_right > mid)，则递归更新右子树。
   4. 左右子树更新返回之后，向上更新节点的区间值（区间和、区间最大值、区间最小值等），区间值等于该节点左右子节点元素值的聚合计算结果。

**2、向下更新:**

延迟标记更新区间的具体步骤为:

1. 更新左子节点值和左子节点的lazy_tag为val.
2. 更新右子节点值和右子节点的lazy_tag为val.
3. 更新当前节点的lazy_tag为None.

其实现代码如下:

```python
    # 区间更新，将区间为 [q_left, q_right] 上的元素值修改为 val
    def update_interval(self, q_left, q_right, val):
        self.__update_interval(q_left, q_right, val, 0, 0, self.size - 1)
        
    # 区间更新
    def __update_interval(self, q_left, q_right, val, index, left, right):
        
        if left >= q_left and right <= q_right:     # 节点所在区间被 [q_left, q_right] 所覆盖
            interval_size = (right - left + 1)      # 当前节点所在区间大小
            self.tree[index].val = interval_size * val # 当前节点所在区间每个元素值改为 val
            self.tree[index].lazy_tag = val         # 将当前节点的延迟标记为区间值
            return
        if right < q_left or left > q_right:        # 节点所在区间与 [q_left, q_right] 无关
            return 0
    
        self.__pushdown(index)
    
        mid = left + (right - left) // 2            # 左右节点划分点
        left_index = index * 2 + 1                  # 左子节点的存储下标
        right_index = index * 2 + 2                 # 右子节点的存储下标
        if q_left <= mid:                           # 在左子树中更新区间值
            self.__update_interval(q_left, q_right, val, left_index, left, mid)
        if q_right > mid:                           # 在右子树中更新区间值
            self.__update_interval(q_left, q_right, val, right_index, mid + 1, right)
        
        self.__pushup(index)
    
    # 向下更新下标为 index 的节点所在区间的左右子节点的值和懒惰标记
    def __pushdown(self, index):
        lazy_tag = self.tree[index].lazy_tag
        if not lazy_tag:
            return
        
        left_index = index * 2 + 1                  # 左子节点的存储下标
        right_index = index * 2 + 2                 # 右子节点的存储下标
        
        self.tree[left_index].lazy_tag = lazy_tag   # 更新左子节点懒惰标记
        left_size = (self.tree[left_index].right - self.tree[left_index].left + 1)
        self.tree[left_index].val = lazy_tag * left_size    # 更新左子节点值

        self.tree[right_index].lazy_tag = lazy_tag  # 更新右子节点懒惰标记
        right_size = (self.tree[right_index].right - self.tree[right_index].left + 1)
        self.tree[right_index].val = lazy_tag * right_size  # 更新右子节点值

        self.tree[index].lazy_tag = None            # 更新当前节点的懒惰标记
```
