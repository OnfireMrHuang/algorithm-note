/*
 * @lc app=leetcode.cn id=139 lang=rust
 *
 * [139] 单词拆分
 */

// @lc code=start
/*
我们假设状态为dp[i] = val; 其中i表示[1..i]的子字符串是否能从字典中的单词拼接而成，val是布尔值。

base case:
dp[0] = true; // 默认0个字符能从字典中的单词拼接而成，用于处理边界情况

动态转移方程:
dp[i] = check(s[j..i]) & dp[j-1] // 我们假设s[j..i]能被单词拼接而成，那么当dp[j-1]即s[0..j-1]也能由单词拼接而成时，那么dp[i]就成立
*/
use std::collections::HashSet;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut set: HashSet<String> = HashSet::new();
        // 先构建哈希集
        for word in word_dict {
            set.insert(word);
        }
        let size = s.len();
        let mut dp = vec![false; size + 1];
        // base case
        dp[0] = true;
        for i in 1..=size {
            for j in 1..=i {
                // 如果dp[i]已经证明过一次能拼成，那么就直接退出了
                if dp[i] {
                    break;
                }
                let is_ok = set.contains(&s[j - 1..i].to_string());
                dp[i] = dp[j - 1] & is_ok;
            }
        }
        dp[size]
    }
}
// @lc code=end
