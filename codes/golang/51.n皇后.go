/*
 * @lc app=leetcode.cn id=51 lang=golang
 *
 * [51] N皇后
 */

 func solveNQueens(n int) [][]string {
	var result [][]string
	board := make([][]rune,n)
	for i:=0; i < n; i++ {
		board[i] = make([]rune,n)
		for j:=0; j < n; j++ {
			board[i][j] = '.'
		}
	}
	backTrace(board, 0, &result)
	return result
}

func backTrace(board [][]rune, row int, result *[][]string) {

	n := len(board)
	if row == n {
		temp := make([]string,n)
		for i:=0; i < n; i++ {
			for j:=0; j < n; j++ {
				temp[i] += string(board[i][j])
			}
		}
		*result = append(*result, temp)
		return
	}
	for col := 0; col < n; col++ {
		if !isValid(board, row, col) {
			//fmt.Println("校验不通过")
			continue
		}
		board[row][col] = 'Q'
		backTrace(board, row+1, result)
		board[row][col] = '.'
	}
}

func isValid(board [][]rune, row int, col int) bool {

	// 这里的行和列表示棋子的坐标
	n := len(board) // 棋盘大小

	// 检查正上方是否有Q
	for i := 0; i < row; i++ {
		if board[i][col] == 'Q' {
			return false
		}
	}

	// 检查右上方有没有皇后
	for i, j := row-1, col+1; i >= 0 && j < n; i, j = i-1, j+1 {
		if board[i][j] == 'Q' {
			return false
		}
	}

	// 检查左上方有没有皇后
	for i, j := row-1, col-1; i >= 0 && j >= 0; i, j = i-1, j-1 {
		if board[i][j] == 'Q' {
			return false
		}
	}

	//// 检查左边有没有Q字符
	//for j:=0; j < col; j=j+1 {
	//	if board[row][j] == 'Q' {
	//		return false
	//	}
	//}
	return true
}

// @lc code=end

