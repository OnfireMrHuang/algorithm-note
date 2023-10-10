/*
 * @lc app=leetcode.cn id=1278 lang=rust
 *
 * [1278] 分割回文串 III
 */

// @lc code=start
impl Solution {
    // 先构造修改次数的动态表: dp[i][j] = value, 状态意思是[0..i]的区间字符串分割为j个区间需要的最小修改数value
    // base case: (单个字符，单个区间其最小修改数等于0)
    // dp[0][0] = 0
    // 区间数应该小于等于字符串的长度，因为分区数不可能比字符数量还要多
    // 动态转移方程:
    // dp[i][j] = dp[m][j-1] + cost(s[m..i])
    // 其中，我们假设了[m..i]是最后一个分区，那么dp[i][j]的最小修改字符数就等于上一个分区(dp[m][j-1])加上最后一个分区需要的最小修改数
    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        let size = s.len();
        let mut cost_table = vec![vec![0; size]; size];
        Self::construct_cost_table(&s, &mut cost_table);
        let mut dp: Vec<Vec<i32>> = vec![vec![i32::MAX; k as usize]; size];
        // base case
        dp[0][0] = 0;
        for i in 1..size {
            for j in 0..=std::cmp::min(i, k as usize - 1) {
                if j == 0 {
                    dp[i][j] = cost_table[0][i];
                } else {
                    // 因为是最后一个分区，说明前面已经进行了j-1次分区，那么就是最少有j-1个字符(1个字符1个分区)
                    for m in j - 1..i {
                        dp[i][j] = std::cmp::min(dp[i][j], dp[m][j - 1] + cost_table[m + 1][i]);
                    }
                }
            }
        }
        dp[s.len() - 1][k as usize - 1]
    }

    // 先构造修改次数的动态表: cost_table[i][j] = value, 状态意思是[i..j]的字符串，需要修改字符value个
    // base case: 如果i>=j, 修改字符数为0
    // 动态转移方程:
    // if s[i] == s[j], 那么cost_table[i][j] = cost_table[i+1][j-1]
    // if s[i] != s[j], 那么cost_table[i][j] = cost_table[i+1][j-1] + 1
    // 从上面的状态转移方程知道: i是降序、j是增序,所以方向是从状态表的最后一行第一列顺序往上遍历
    fn construct_cost_table(s: &str, cost_table: &mut Vec<Vec<i32>>) {
        let size = s.len();
        for i in (0..size).rev() {
            // j需要>=i, 超出j的i没有任何意义
            for j in i..size {
                if i == j {
                    cost_table[i][j] = 0
                } else {
                    if &s[i..i + 1] == &s[j..j + 1] {
                        cost_table[i][j] = cost_table[i + 1][j - 1];
                    } else {
                        cost_table[i][j] = cost_table[i + 1][j - 1] + 1;
                    }
                }
            }
        }
    }
}
// @lc code=end
