# 题解

## 20.有效的括号

> [题目描述](https://leetcode-cn.com/problems/valid-parentheses/)

**题目解法**: 该题目使用栈来解决，当遇到左括号时入栈`stack`，遇到右括号时出栈`stack`，对比是否左右匹配(如果不匹配那就无效)；最后再查看`stack`是否还有括号，没有则有效,否则无效.

[rust版本](../../../codes/rust/20.有效的括号.rs) |
[java版本](../../../codes/java/20.有效的括号.java) |
[golang版本](../../../codes/golang/20.有效的括号.go) |
[python版本](../../../codes/python/20.有效的括号.py)

<br>

## 224.基本计算器

> [题目描述](https://leetcode-cn.com/problems/basic-calculator/)

**题目解法**:

下面是`labuladong`的题解思路，很清晰，直接引用了:
[计算器解题思路](https://labuladong.github.io/algo/di-san-zha-24031/jing-dian--a94a0/ru-he-shi--24fe4/)

[rust版本](../../../codes/rust/224.基本计算器.rs) |
[java版本](../../../codes/java/224.基本计算器.java) |
[golang版本](../../../codes/golang/224.基本计算器.go) |
[python版本](../../../codes/python/224.基本计算器.py)

<br>

## 341.扁平化嵌套列表迭代器

> [题目描述](https://leetcode-cn.com/problems/flatten-nested-list-iterator/)

**题目解法**: 该题目我们知道输入的是一个嵌套数值列表，当我们迭代遍历一个列表时，如果遇到的是一个数字，则输出数字，如果遇到的是一个列表，则可以递归该列表将其展平为数字列表，再输出第一个数字。

我们可以利用hasNext方法，对列表优先展平，这样在next方法中，我们就可以直接输出第一个数字了。

[rust版本](../../../codes/rust/341.扁平化嵌套列表迭代器.rs) |
[java版本](../../../codes/java/341.扁平化嵌套列表迭代器.java) |
[golang版本](../../../codes/golang/341.扁平化嵌套列表迭代器.go) |
[python版本](../../../codes/python/341.扁平化嵌套列表迭代器.py)

<br>

## 621.任务调度器

> [题目描述](https://leetcode-cn.com/problems/task-scheduler/)

**题目解法**: 我们可以考虑先将同类型的`task`放入到一个队列中，这样就有多个不同类型任务的队列，然后对比`n+1`和队列数的大小(之所以是n+1是因为n是间隔大小，就比如1和3的间隔是1，但是1和3的差值是2):

- 如果`n+1`大于等于队列数，那么最短时间就是n * (最大的队列高度-1) + 最大队列的高度一层的剩余任务数.(同一类任务必须间隔n，所以除了最后一层，其余层都是固定的`n+1`)
- 如果`n+1`小于队列数，我们可以将超过`n+1`的队列中的任务插入到其他队列中, 此时会出现两种情况:
  - 第一种情况: 插入队列后的任务均匀分布，那么最短时间就是`tasks.len()`
  - 第二种情况: 插入队列后的任务不均匀分布，比如某一类任务数远大于其他类任务，那么最短时间就是`n * (最大的队列高度-1) + 最大队列的高度一层的剩余任务数`

示例图:



[rust版本](../../../codes/rust/621.任务调度器.rs) |
[java版本](../../../codes/java/621.任务调度器.java) |
[golang版本](../../../codes/golang/621.任务调度器.go) |
[python版本](../../../codes/python/621.任务调度器.py)