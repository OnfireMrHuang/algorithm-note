/*
 * @lc app=leetcode.cn id=131 lang=golang
 *
 * [131] 分割回文串
 *
 * https://leetcode-cn.com/problems/palindrome-partitioning/description/
 *
 * algorithms
 * Medium (61.94%)
 * Total Accepted:    3.7K
 * Total Submissions: 6K
 * Testcase Example:  '"aab"'
 *
 * 给定一个字符串 s，将 s 分割成一些子串，使每个子串都是回文串。
 *
 * 返回 s 所有可能的分割方案。
 *
 * 示例:
 *
 * 输入: "aab"
 * 输出:
 * [
 * ⁠ ["aa","b"],
 * ⁠ ["a","a","b"]
 * ]
 *
 */

// s 为回文，则返回 true
func par(s string) bool {
	if len(s) <= 1 {
		return true
	}
	a, b := 0, len(s)-1
	for a < b {
		if s[a] != s[b] {
			return false
		}
		a++
		b--
	}
	return true
}

func dfs(s string, i int, cur []string, result *[][]string) {
	if i == len(s) {
		//分割索引到达了结尾,一次划分结束
		tmp := make([]string, len(cur))
		copy(tmp, cur)
		*result = append(*result, tmp)
		return
	}

	for j := i; j < len(s); j++ {
		if par(s[i : j+1]) {
			dfs(s, j+1, append(cur, s[i:j+1]), result)
		}
	}
}

func partition(s string) [][]string {
	res := [][]string{}
	cur := make([]string, 0, len(s))
	dfs(s, 0, cur, &res)
	return res
}
