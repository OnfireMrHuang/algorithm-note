# 树

## 二叉树与多叉树

二叉树是一种非线性数据结构, 由节点组成, 每个节点最多有两个子节点, 分别为左子节点和右子节点.
而多叉树则是每个节点有多个子节点.
其实链表也是一种特殊的树, 只不过每个节点只有一个子节点.

树的常用术语:

- 「根节点 root node」：位于二叉树顶层的节点，没有父节点。
- 「叶节点 leaf node」：没有子节点的节点，其两个指针均指向 None 。
- 「边 edge」：连接两个节点的线段，即节点引用（指针）。
- 节点所在的「层 level」：从顶至底递增，根节点所在层为 1 。
- 节点的「度 degree」：节点的子节点的数量。在二叉树中，度的取值范围是 0、1、2 。
- 树的「高度 height」：从根节点到最远叶节点所经过的边的数量。
- 节点的「深度 depth」：从根节点到该节点所经过的边的数量。
- 节点的「高度 height」：从距离该节点最远的叶节点到该节点所经过的边的数量。

![tree](https://www.hello-algo.com/chapter_tree/binary_tree.assets/binary_tree_terminology.png)

## 树的遍历

### 广度优先遍历

 广度优先遍历的意思是从根节点开始, 从上到下, 从左到右依次遍历每个节点.

![bfs](https://www.hello-algo.com/chapter_tree/binary_tree_traversal.assets/binary_tree_bfs.png)

示例代码表示:

```python
def level_order(root: TreeNode | None) -> list[int]:
    """层序遍历"""
    # 初始化队列，加入根节点
    queue: deque[TreeNode] = deque()
    queue.append(root)
    # 初始化一个列表，用于保存遍历序列
    res = []
    while queue:
        node: TreeNode = queue.popleft()  # 队列出队
        ......  # 访问 node 节点,执行相关操作
        if node.left is not None:
            queue.append(node.left)  # 左子节点入队
        if node.right is not None:
            queue.append(node.right)  # 右子节点入队
    return res
```

### 深度优先遍历

 深度优先遍历又分为前序遍历, 中序遍历和后序遍历.它们遍历的方式都是从根节点开始, 从上到下, 先走到叶子节点，再往上回溯遍历其他节点的遍历方式.

![dfs](https://www.hello-algo.com/chapter_tree/binary_tree_traversal.assets/binary_tree_dfs.png)

示例代码表示:

```python
def pre_order(root: TreeNode | None):
    """前序遍历"""
    if root is None:
        return
    # 访问优先级：根节点 -> 左子树 -> 右子树
    res.append(root.val)
    pre_order(root=root.left)
    pre_order(root=root.right)

def in_order(root: TreeNode | None):
    """中序遍历"""
    if root is None:
        return
    # 访问优先级：左子树 -> 根节点 -> 右子树
    in_order(root=root.left)
    res.append(root.val)
    in_order(root=root.right)

def post_order(root: TreeNode | None):
    """后序遍历"""
    if root is None:
        return
    # 访问优先级：左子树 -> 右子树 -> 根节点
    post_order(root=root.left)
    post_order(root=root.right)
    res.append(root.val)
```

## 二叉树的数组表示

通过广度优先遍历从上往下，从左边往右的特性可知，我们可以遍历的节点顺序作为数组索引来追加到数组中，这样就可以将二叉树转换为数组表示。

那么很容易的就推导出，对于任意一个节点的索引为 i ，其左子节点索引为 2i+1 ，右子节点索引为 2i+2 。如下图所示：

![tree_to_array](https://www.hello-algo.com/chapter_tree/array_representation_of_tree.assets/array_representation_binary_tree.png)

当然，上述例子中的二叉树是一颗完全二叉树，对于一般的二叉树，我们可以用 None 来表示空节点，这样就可以将任意二叉树转换为数组表示。

## 二叉搜索树与二叉平衡树

在数据存储与检索场景中，二叉搜索树是一种常用的数据结构，它的特点是：

- 左子树上所有节点的值均小于根节点的值
- 右子树上所有节点的值均大于根节点的值
  
二叉搜索树对于我们检索数据是非常有用的，当我们需要检索一个值时，只需要从根节点开始，如果当前节点的值大于待检索值，则继续检索左子树，否则继续检索右子树，直到找到该值或者检索到空节点。整体的检索过程类似于二分查找，时间复杂度为 O(logn)。

![bst_search](https://www.hello-algo.com/chapter_tree/binary_search_tree.assets/bst_search_step4.png)

但是，二叉搜索树也有一个缺点，就是当我们插入的数据是有序的时候，二叉搜索树会退化成一个链表，此时检索的时间复杂度就会退化成 O(n)。

![bst_degradation](https://www.hello-algo.com/chapter_tree/binary_search_tree.assets/bst_degradation.png)

于是为了针对这种退化场景，于是就引出了二叉平衡树，它的特点是：

- 左右子树的高度差不超过 1
- 左右子树也是一颗平衡树
- 二叉平衡树的高度为 O(logn)
- 二叉平衡树的插入、删除、查找的时间复杂度为 O(logn)

其中，保持左右子树高度差不超过 1 的操作称为「平衡操作」，常见的平衡操作有「左旋」和「右旋」,具体原理可参考[AVL树旋转](https://www.hello-algo.com/chapter_tree/avl_tree/#752-avl)。

实现二叉平衡树的方式有很多，比如 AVL树、红黑树等，详情请参考[平衡二叉树](https://zh.wikipedia.org/wiki/%E5%B9%B3%E8%A1%A1%E4%BA%8C%E5%85%83%E6%90%9C%E5%B0%8B%E6%A8%B9)

## B树与B+树

二叉平衡树的缺点是每个节点只有两个子节点，这样就会导致树的高度比较高，当平衡二叉树完全在内存中构建和访问时，访问效率还是比较高的，但是当平衡二叉树存储在磁盘中时，由于磁盘的读写单位是页，而页的大小一般为 4KB，所以每次读取的数据量为 4KB，而平衡二叉树的节点大小一般是远不足 4KB 的，这就导致页的利用率很低，需要读取多次磁盘才能完成一次检索，这样就会导致检索效率降低。于是就引出了 B树和 B+树。

### B树

> [B树博文](https://oi-wiki.org/ds/b-tree/) 、[B树wiki](https://zh.wikipedia.org/wiki/B%E6%A0%91)

- B树有两种节点: 
  - 内部节点: 存储了数据以及指向了其子节点的指针
  - 叶子节点: 叶子结点只存储了数据，没有指向子节点的指针
- 如果根节点不是叶子节点，那么它至少有两个子节点
- 有`k`个子节点的非叶子节点拥有`k−1`个键，且升序排列，满足`k[i] < k[i+1]`。
- 每个节点至多包含`2k-1`个键。
- 所有的叶子节点都在同一层.

![b-tree](https://oi-wiki.org/ds/images/b-tree-1.svg)

B树的基本操作有查找、遍历、插入和删除，其中详细的实现过程可参考上述的引用博文。其中，插入和删除会涉及到节点的分裂和合并，可以重点关注。

### B+树

> [B+树博文](https://oi-wiki.org/ds/bplus-tree/)、[B+树wiki](https://zh.wikipedia.org/wiki/B%2B%E6%A0%91)

B+树是对B树的一个升级，现在现代关系型数据库中应用最广泛的索引数据结构就是B+树。

B+树与B树的区别在于：

- B+树的非叶子节点不存储数据，只存储键值，这样可以使得B+树的非叶子节点更小，可以存储更多的键值，这样就可以使得B+树的高度更低，从而减少磁盘的读写次数，提高检索效率。
- B+ 树为了方便范围查询，叶子节点之间还用指针串联起来

图示如下:

![b+tree](https://oi-wiki.org/ds/images/bplus-tree-1.png)
