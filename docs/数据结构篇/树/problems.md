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

**题目解法:** xx

[rust版本](../../../codes/rust/105.从前序与中序遍历序列构造二叉树.rs) |
[java版本](../../../codes/java/105.从前序与中序遍历序列构造二叉树.java) |
[golang版本](../../../codes/golang/105.从前序与中序遍历序列构造二叉树.go) |
[python版本](../../../codes/python/105.从前序与中序遍历序列构造二叉树.py)

</br>

## 112. 路径总和

> [题目描述](https://leetcode-cn.com/problems/path-sum/)

</br>

## 113. 路径总和 II

> [题目描述](https://leetcode-cn.com/problems/path-sum-ii/)

</br>

## 437. 路径总和 III

> [题目描述](https://leetcode-cn.com/problems/path-sum-iii/)

</br>

## 124. 二叉树中的最大路径和

> [题目描述](https://leetcode-cn.com/problems/binary-tree-maximum-path-sum/)

</br>

## 226. 翻转二叉树

> [题目描述](https://leetcode-cn.com/problems/invert-binary-tree/)

</br>

## 235. 二叉搜索树的最近公共祖先

> [题目描述](https://leetcode-cn.com/problems/lowest-common-ancestor-of-a-binary-search-tree/)

</br>

## 236. 二叉树的最近公共祖先

> [题目描述](https://leetcode-cn.com/problems/lowest-common-ancestor-of-a-binary-tree/)

</br>

## 449. 序列化和反序列化二叉搜索树

> [题目描述](https://leetcode-cn.com/problems/serialize-and-deserialize-bst/)

</br>

## 743. 网络延迟时间

> [题目描述](https://leetcode-cn.com/problems/network-delay-time/)

</br>
