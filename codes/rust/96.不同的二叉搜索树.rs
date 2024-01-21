// @lc code=start
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut memo = vec![vec![0; n as usize + 1]; n as usize + 1];
        Self::count(1, n, &mut memo)
    }

    fn count(lo: i32, hi: i32, memo: &mut Vec<Vec<i32>>) -> i32 {
        if lo > hi {
            return 1;
        }
        if memo[lo as usize][hi as usize] != 0 {
            return memo[lo as usize][hi as usize];
        }
        let mut res = 0;
        for mid in lo..=hi {
            let left = Self::count(lo, mid - 1, memo);
            let right = Self::count(mid + 1, hi, memo);
            res += left * right;
        }
        memo[lo as usize][hi as usize] = res;
        res
    }
}
// @lc code=end
