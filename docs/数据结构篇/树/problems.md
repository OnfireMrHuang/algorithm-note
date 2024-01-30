# 题解

## 94. 二叉树的中序遍历

> [题目描述](https://leetcode-cn.com/problems/binary-tree-inorder-traversal/)

**题目解法:** 该题目正如简要说明中深度优先遍历的一样，先访问左子树，再访问根节点，最后访问右子树。因此，我们可以使用递归的方式来实现该题目。

[rust版本](../../../codes/rust/94.二叉树的中序遍历.rs) |
[java版本](../../../codes/java/94.二叉树的中序遍历.java) |
[golang版本](../../../codes/golang/94.二叉树的中序遍历.go) |
[python版本](../../../codes/python/94.二叉树的中序遍历.py)

</br>

## 96. 不同的二叉搜索树

> [题目描述](https://leetcode-cn.com/problems/unique-binary-search-trees/)

**题目解法:** 当`n==0`时，互不相同的二叉树有零种，当`n==1`时，互不相同的二叉树只有一种，当`n==2`时，互不相同的有两种。当`n==5`时，如果我们选择3为根节点，那么左子树由`1和2`组成，右子树由`4和5`组成，也就是说当我们固定其中一个数字作为根节点，那么就去递归统计左右子树可能的组合，一直到左右子树<=2，直到n个数字每个都作为根节点统计一遍，最后将所有的结果相加即可。为了性能优化，我们可以使用备忘录的方式来记录已经计算过的结果，避免重复计算。

[rust版本](../../../codes/rust/96.不同的二叉搜索树.rs) |
[java版本](../../../codes/java/96.不同的二叉搜索树.java) |
[golang版本](../../../codes/golang/96.不同的二叉搜索树.go) |
[python版本](../../../codes/python/96.不同的二叉搜索树.py)

</br>

## 98. 验证二叉搜索树

> [题目描述](https://leetcode-cn.com/problems/validate-binary-search-tree/)

**题目解法:** 根据题目要求，该题目可以使用前序遍历的方式来判断，当为根节点时我们假设其取值范围为[-∞, +∞]，当其左子树不为空时，其取值范围为[-∞, root.val]，当其右子树不为空时，其取值范围为[root.val, +∞]。通过该方式我们判断每个节点的值是否在其取值范围内即可。

[rust版本](../../../codes/rust/98.验证二叉搜索树.rs) |
[java版本](../../../codes/java/98.验证二叉搜索树.java) |
[golang版本](../../../codes/golang/98.验证二叉搜索树.go) |
[python版本](../../../codes/python/98.验证二叉搜索树.py)

</br>

## 99. 恢复二叉搜索树

> [题目描述](https://leetcode-cn.com/problems/recover-binary-search-tree/)

**题目解法:** 改题目描述二叉树中恰好存在一对不符合的节点导致不能成为平衡二叉树，只要通过交换这两个节点就可以使其成为平衡二叉树。我们可以利用中序遍历的方式来遍历二叉树，因为二叉搜索树的中序遍历是有序的，通过中序遍历得到一个有序数组，然后找到其中不符合升序的两个节点，然后再二叉数中交换这两个节点值即可。

[rust版本](../../../codes/rust/99.恢复二叉搜索树.rs) |
[java版本](../../../codes/java/99.恢复二叉搜索树.java) |
[golang版本](../../../codes/golang/99.恢复二叉搜索树.go) |
[python版本](../../../codes/python/99.恢复二叉搜索树.py)

</br>

## 102. 二叉树的层序遍历

> [题目描述](https://leetcode-cn.com/problems/binary-tree-level-order-traversal/)

**题目解法:** 该题目是典型的广度优先遍历实现，具体原理可以参考[广度优先遍历](./brief_introduction.md)。

[rust版本](../../../codes/rust/102.二叉树的层序遍历.rs) |
[java版本](../../../codes/java/102.二叉树的层序遍历.java) |
[golang版本](../../../codes/golang/102.二叉树的层序遍历.go) |
[python版本](../../../codes/python/102.二叉树的层序遍历.py)

</br>

## 104. 二叉树的最大深度

> [题目描述](https://leetcode-cn.com/problems/maximum-depth-of-binary-tree/)

**题目解法:** 该题目相对比较简单，只需要通过对比左右子树的深度，然后取最大值即可。(叶子节点的深度为1)

[rust版本](../../../codes/rust/104.二叉树的最大深度.rs) |
[java版本](../../../codes/java/104.二叉树的最大深度.java) |
[golang版本](../../../codes/golang/104.二叉树的最大深度.go) |
[python版本](../../../codes/python/104.二叉树的最大深度.py)

</br>

## 105. 从前序与中序遍历序列构造二叉树

> [题目描述](https://leetcode-cn.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/)

**题目解法:** 构造二叉树，我们通常都是先找到一个根节点，然后接着再找左右子树的根节点并设置，以此类推，直到递归找到所有节点。通过前序遍历我们知道，根节点一定第一个元素，接着是左子树的元素，再然后是右子树的元素；而中序遍历是先左子树，再根节点，最后右子树。于是我们可以通过前序遍历找到根节点，然后再通过中序遍历的映射找到左右子树的集合，通过左右子树的集合找到左右根节点，然后再递归往下找左右子树的根节点，直到找到所有节点。整个过程就是一个不断切分左右子树的过程。

[rust版本](../../../codes/rust/105.从前序与中序遍历序列构造二叉树.rs) |
[java版本](../../../codes/java/105.从前序与中序遍历序列构造二叉树.java) |
[golang版本](../../../codes/golang/105.从前序与中序遍历序列构造二叉树.go) |
[python版本](../../../codes/python/105.从前序与中序遍历序列构造二叉树.py)

</br>

## 112. 路径总和

> [题目描述](https://leetcode-cn.com/problems/path-sum/)

**题目解法:** 该题目比较简单，我们只需要通过将目标值减去当前的节点值作为新的目标值，递归传递到左右子树，直到叶子节点，然后判断叶子节点的值是否等于目标值即可。

[rust版本](../../../codes/rust/112.路径总和.rs) |
[java版本](../../../codes/java/112.路径总和.java) |
[golang版本](../../../codes/golang/112.路径总和.go) |
[python版本](../../../codes/python/112.路径总和.py)

</br>

## 113. 路径总和 II

> [题目描述](https://leetcode-cn.com/problems/path-sum-ii/)

**题目解法:** 这题是上一题的升级版，我们只需要在路径总和的基础上，做如下改动即可:

1. 将路径和结果数组作为参数传递下去，每次递归时将当前节点加入到路径中
2. 当叶子节点的值等于目标值时，将当前路径加入到结果数组中
3. 递归返回时，将当前节点从路径中移除

[rust版本](../../../codes/rust/113.路径总和-ii.rs) |
[java版本](../../../codes/java/113.路径总和-ii.java) |
[golang版本](../../../codes/golang/113.路径总和-ii.go) |
[python版本](../../../codes/python/113.路径总和-ii.py)

</br>

## 437. 路径总和 III

> [题目描述](https://leetcode-cn.com/problems/path-sum-iii/)

**题目解法:** 该题目同样使用将路径作为可变更参数递归传递下去，以及回溯时将当前节点从路径中移除的方式。当递归遍历到一个节点时(不管是否是分支节点还是叶子节点), 首先判断当前节点的值是否等于目标值，如果等于则将结果加一，然后再与路径逆序相加，判断是否等于目标值，如果等于则将结果加一，然后再递归遍历左右子树，将左右子树的结果与当前结果相加后返回即可。

[rust版本](../../../codes/rust/437.路径总和-iii.rs) |
[java版本](../../../codes/java/437.路径总和-iii.java) |
[golang版本](../../../codes/golang/437.路径总和-iii.go) |
[python版本](../../../codes/python/437.路径总和-iii.py)

</br>

## 124. 二叉树中的最大路径和

> [题目描述](https://leetcode-cn.com/problems/binary-tree-maximum-path-sum/)

**题目解法:** todo: 最后实现

[rust版本](../../../codes/rust/124.二叉树中的最大路径和.rs) |
[java版本](../../../codes/java/124.二叉树中的最大路径和.java) |
[golang版本](../../../codes/golang/124.二叉树中的最大路径和.go) |
[python版本](../../../codes/python/124.二叉树中的最大路径和.py)

</br>

## 226. 翻转二叉树

> [题目描述](https://leetcode-cn.com/problems/invert-binary-tree/)

**题目解法:** 该题目直接使用递归的方式先翻转左右子树，然后再交换左右子树即可。

[rust版本](../../../codes/rust/226.翻转二叉树.rs) |
[java版本](../../../codes/java/226.翻转二叉树.java) |
[golang版本](../../../codes/golang/226.翻转二叉树.go) |
[python版本](../../../codes/python/226.翻转二叉树.py)

</br>

## 235. 二叉搜索树的最近公共祖先

> [题目描述](https://leetcode-cn.com/problems/lowest-common-ancestor-of-a-binary-search-tree/)

**题目解法:** todo

[rust版本](../../../codes/rust/235.二叉搜索树的最近公共祖先.rs) |
[java版本](../../../codes/java/235.二叉搜索树的最近公共祖先.java) |
[golang版本](../../../codes/golang/235.二叉搜索树的最近公共祖先.go) |
[python版本](../../../codes/python/235.二叉搜索树的最近公共祖先.py)

</br>

## 236. 二叉树的最近公共祖先

> [题目描述](https://leetcode-cn.com/problems/lowest-common-ancestor-of-a-binary-tree/)

**题目解法:** todo

[rust版本](../../../codes/rust/236.二叉树的最近公共祖先.rs) |
[java版本](../../../codes/java/236.二叉树的最近公共祖先.java) |
[golang版本](../../../codes/golang/236.二叉树的最近公共祖先.go) |
[python版本](../../../codes/python/236.二叉树的最近公共祖先.py)

</br>

## 449. 序列化和反序列化二叉搜索树

> [题目描述](https://leetcode-cn.com/problems/serialize-and-deserialize-bst/)

**题目解法:** todo

[rust版本](../../../codes/rust/449.序列化和反序列化二叉搜索树.rs) |
[java版本](../../../codes/java/449.序列化和反序列化二叉搜索树.java) |
[golang版本](../../../codes/golang/449.序列化和反序列化二叉搜索树.go) |
[python版本](../../../codes/python/449.序列化和反序列化二叉搜索树.py)

</br>

## 743. 网络延迟时间

> [题目描述](https://leetcode-cn.com/problems/network-delay-time/)

**题目解法:** todo

[rust版本](../../../codes/rust/743.网络延迟时间.rs) |
[java版本](../../../codes/java/743.网络延迟时间.java) |
[golang版本](../../../codes/golang/743.网络延迟时间.go) |
[python版本](../../../codes/python/743.网络延迟时间.py)

</br>
