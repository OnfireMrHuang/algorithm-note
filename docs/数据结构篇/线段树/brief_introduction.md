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
   1. 


### 区间更新

## 线段树的应用


