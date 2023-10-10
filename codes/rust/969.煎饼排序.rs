/*
 * @lc app=leetcode.cn id=969 lang=rust
 *
 * [969] 煎饼排序
 */

// @lc code=start
impl Solution {
    pub fn pancake_sort(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        let mut n = arr.len() as i32;
        let mut ans: Vec<i32> = Vec::new();
        // 首先我们反向构建值到下标的映射
        let mut index: Vec<usize> = vec![0; n as usize + 1];
        for (i, val) in arr.iter().enumerate() {
            index[*val as usize] = i;
        }
        // 接着我们从大到小遍历烧饼，将最大的烧饼翻转到最下面
        for i in (1..=n).rev() {
            // 取出最大值的下标
            let idx = index[i as usize];
            // 如果下标刚好相等，则说明本身是有序的，不需要翻转
            if idx == i as usize - 1 {
                continue;
            }
            // 如果不等于0，我们需要先把烧饼翻转到最上面
            if idx != 0 {
                ans.push(idx as i32 + 1);
                Self::reverse(&mut arr, 0, idx);
            }
            // 再将第一个烧饼翻转到i-1这个位置
            ans.push(i);
            Self::reverse(&mut arr, 0, i as usize - 1);
        }

        ans
    }

    fn reverse(cakes: &mut Vec<i32>, i: usize, j: usize) {
        let (mut i, mut j) = (i, j);
        while i < j {
            let tmp = cakes[i];
            cakes[i] = cakes[j];
            cakes[j] = tmp;
            i += 1;
            j -= 1;
        }
    }
}
// @lc code=end
