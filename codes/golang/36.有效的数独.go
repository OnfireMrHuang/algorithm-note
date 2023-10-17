/*
 * @lc app=leetcode.cn id=36 lang=golang
 *
 * [36] 有效的数独
 */

// @lc code=start
func isValidSudoku(board [][]byte) bool {
	rowbuf,colbuf,boxbuf := make([][]bool,9),make([][]bool,9),make([][]bool,9)
	for i:=0; i<9;i++ {
		rowbuf[i] = make([]bool,9)
		colbuf[i] = make([]bool,9)
		boxbuf[i] = make([]bool,9)
	}
	// 遍历一次，添加缓存
	for r:=0;r<9;r++ {
		for c:=0;c<9;c++ {
			if board[r][c] != '.' {
				num := board[r][c] - '0' - byte(1)
				if rowbuf[r][num] || colbuf[c][num] || boxbuf[r/3*3+c/3][num] {
					return false
				}
				rowbuf[r][num] = true
				colbuf[c][num] = true
				boxbuf[r/3*3+c/3][num] = true
			}
		}
	}
	return true
}
// @lc code=end

