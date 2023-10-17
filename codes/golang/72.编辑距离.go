/*
 * @lc app=leetcode.cn id=72 lang=golang
 *
 * [72] 编辑距离
 */

/*
动态规划解法:
base case:  字符串1或字符串2走完，这个时候的base case是另外一个字符串剩下
的长度
状态的转移： 遍历s1的下标递增或s2的下标递增
方法： 自顶向下
状态变换方向： 以字符串1为例子，s1插入一个字符、删除一个字符、替换一个字符
*/

// @lc code=start
func minDistance(word1 string, word2 string) int {
	m := len(word1)
	n := len(word2)

	// 创建dp数组
	mp := make([][]int,m+1)
    mp[0] = make([]int,n+1)
	for i:=1; i < m+1; i++ {
		mp[i] = make([]int, n+1)
		mp[i][0] = i
	}
	for j:=1; j < n+1; j++ {
		mp[0][j] = j
	}

	// 自顶向下求解
	for i:=1; i <=m; i++ {
		for j:=1; j<=n; j++ {
			if word1[i-1] == word2[j-1] {
				mp[i][j] = mp[i-1][j-1]
			} else {
				mp[i][j] = mins(
					mp[i-1][j-1] + 1,
					mp[i-1][j] + 1,
					mp[i][j-1] +1,
				)
			}
		}
	} 
	return mp[m][n]
}

func mins(a,b,c int) int {
	return min(a,min(b,c))
}

func min(a,b int) int {
	if a > b {
		return b
	}
	return a
}

// @lc code=end

