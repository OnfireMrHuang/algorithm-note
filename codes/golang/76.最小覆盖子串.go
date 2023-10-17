/*
 * @lc app=leetcode.cn id=76 lang=golang
 *
 * [76] 最小覆盖子串
 */

 import "math"

// @lc code=start
func minWindow(s string, t string) string {
	need := make(map[byte]int)
	window := make(map[byte]int)

	// 先缓存字符串t中的字符和出现次数
	for i:=0; i < len(t); i++ {
		need[t[i]]++
	}
	// fmt.Printf("need: %v \n",need)
	left,right,valid := 0,0,0
	start,size := 0,math.MaxInt32

	for right < len(s) {

		c := s[right]

		right++ // 右移窗口
		//右移过程中对t中的数据出现次数累加
		if _,ok := need[c]; ok {
			window[c]++
			// 如果need中的出现次数和窗口中的出现次数相同的话就表示已经覆盖了
			if window[c] == need[c] {
				valid++
			}
		}

		// 判断是否已经覆盖完字串了
		for valid == len(need) {
			// 更新窗口起始位置和大小
			if right - left < size {
				start = left
				size = right - left
			}
			// 取出要移除掉的字符
			d := s[left]
			left++
			// 如果移除掉的字符存在
			if _,ok :=need[d]; ok {
				// 开始不含字串了，把valid值减1
				if window[d] == need[d] {
					valid--					
				}
				window[d]--
			}
		}
	}
	// 返回最小覆盖子串
	if size == math.MaxInt32 {
		return ""
	}
	return s[start:(start+size)]
}
// @lc code=end

