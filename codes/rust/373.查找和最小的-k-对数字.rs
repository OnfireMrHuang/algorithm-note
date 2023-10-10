/*
 * @lc app=leetcode.cn id=373 lang=rust
 *
 * [373] 查找和最小的 K 对数字
 */

// @lc code=start
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Pair {
    i: usize,
    num1: i32,
    j: usize,
    num2: i32,
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.num1 + self.num2).partial_cmp(&(other.num1 + other.num2))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.num1 + self.num2).cmp(&(other.num1 + other.num2))
    }
}

impl Solution {
    /*
    该题目等于是求两个数组的所有双值排列，然后取前k个最小的双值排列.
    该题目第一印象就是使用堆排序，即用优先级队列存储所有的双值排列，然后取前k个最小的双值排列.
    上面的算法会超时,优化解法直接参考leetcode官方题解:
    https://leetcode.cn/problems/find-k-pairs-with-smallest-sums/solutions/1208350/cha-zhao-he-zui-xiao-de-kdui-shu-zi-by-l-z526/
    */
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut piror_queue = BinaryHeap::new();
        // 初始化优先级队列
        let m = nums1.len();
        let n = nums2.len();
        for i in 0..std::cmp::min(m, k as usize) {
            piror_queue.push(Reverse(Pair {
                i,
                num1: nums1[i],
                j: 0,
                num2: nums2[0],
            }));
        }
        // 从优先级队列弹，第一个肯定是(0,0)下标的
        while !piror_queue.is_empty() && ans.len() < k as usize {
            let pair = piror_queue.pop().unwrap().0;
            ans.push(vec![pair.num1, pair.num2]);
            // 将下一个元素加入到优先级队列中
            if pair.j + 1 < n {
                piror_queue.push(Reverse(Pair {
                    i: pair.i,
                    num1: pair.num1,
                    j: pair.j + 1,
                    num2: nums2[pair.j + 1],
                }));
            }
        }
        ans
    }
}
// @lc code=end
