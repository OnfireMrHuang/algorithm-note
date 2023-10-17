/*
 * @lc app=leetcode.cn id=22 lang=golang
 *
 * [22] 括号生成
 */

// @lc code=start
func generateParenthesis(n int) []string {
	if n == 0{
		return []string{}
	}
	res := []string{}
	findGenerateParenthesis(n,n,"",&res)
	return res
}

func findGenerateParenthesis(lindex,rindex int,str string,res *[]string) {
	if lindex == 0 && rindex == 0 {
		*res = append(*res,str)
	}
	if lindex > 0 {
		findGenerateParenthesis(lindex-1,rindex,str+"(",res)
	}
	if rindex > 0 && lindex < rindex {
		findGenerateParenthesis(lindex,rindex-1,str+")",res)
	}
}
// @lc code=end

