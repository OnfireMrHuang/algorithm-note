/*
 * @lc app=leetcode.cn id=567 lang=golang
 *
 * [567] 字符串的排列
 */

 import "math"

// @lc code=start
func checkInclusion(s1 string, s2 string) bool {
	need := make(map[byte]int)
	window := make(map[byte]int)

	left,right,valid := 0,0,0

	for i:=0; i<len(s1); i++ {
		need[s1[i]]++
	}

	for right < len(s2) {
		c := s2[right]

		right++
		if _,ok := need[c];ok {
			window[c]++
			if window[c] == need[c] {
				valid++
			}			
		}
		for right - left >= len(s1) {
			d := s2[left]
			left++
			if valid == len(need) {
				return true
			}
			if _,ok := need[d];ok {
				if window[d] == need[d] {
					valid--
				}
				window[d]--
			}
		}
	}
	return false
}
// @lc code=end

