/*
 * @lc app=leetcode.cn id=438 lang=golang
 *
 * [438] 找到字符串中所有字母异位词
 */

// @lc code=start
func findAnagrams(s string, p string) []int {
	need := make(map[byte]int)
	window := make(map[byte]int)

	left,right,valid := 0,0,0

	var result []int

	for i:=0; i<len(p); i++ {
		need[p[i]]++
	}

	for right < len(s) {
		c := s[right]

		right++
		if _,ok := need[c];ok {
			window[c]++
			if window[c] == need[c] {
				valid++
			}			
		}
		for right - left >= len(p) {
			if valid == len(need) {
				result = append(result,left)
			}
			d := s[left]
			left++
			if _,ok := need[d];ok {
				if window[d] == need[d] {
					valid--
				}
				window[d]--
			}
		}
	}
	return result
}
// @lc code=end

