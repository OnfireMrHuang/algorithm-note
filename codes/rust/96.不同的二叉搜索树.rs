/*
 * @lc app=leetcode.cn id=96 lang=rust
 *
 * [96] 不同的二叉搜索树
 */

// @lc code=start
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        if n <= 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 2;
        }
        let mut g: Vec<i32> = Vec::with_capacity(n as usize);
        g.push(1);
        g.push(1);
        for i in 3..=n + 1 {
            let mut num = 0;
            for j in 1..i {
                num += g[(j - 1) as usize] * g[(i - j - 1) as usize];
            }
            g.push(num);
        }
        g.pop().unwrap()
    }
}
// @lc code=end
