/*
 * @lc app=leetcode.cn id=52 lang=golang
 *
 * [52] N皇后 II
 */

// @lc code=start
func totalNQueens(n int) int {
	col,dia1,dia2,res := make([]bool,n),make([]bool,2*n-1),make([]bool,2*n-1),0
	putQueen(n,0,&col,&dia1,&dia2,&res)
	return res
}

func putQueen(n,index int,col,dia1,dia2 *[]bool,res *int) {
	if index == n {
		*res += 1
		return
	}
	for i:=0;i<n;i++ {
		if !(*col)[i] && !(*dia1)[index+i] && !(*dia2)[index-i+n-1] {
			(*col)[i] = true
			(*dia1)[index+i] = true
			(*dia2)[index-i+n-1] = true
			putQueen(n,index+1,col,dia1,dia2,res)
			(*col)[i]=false
			(*dia1)[index+i]=false
			(*dia2)[index-i+n-1]=false
		}
	}
}

// var count int 

// func totalNQueens(n int) int {
// 	if n == 1 {
// 		return 1
// 	}
// 	putQueen(n,0,0,0,0)
// 	return count
// }

// func putQueen(n,row,col,pia,na int) {
// 	if row >= n {
// 		count++
// 		return
// 	}

// 	// 查看是否有空位
// 	bits := (^(col | pia | na)) & ((1<<n)-1)
// 	for bits > 0 {
// 		// 取出最近的一个空位
// 		p := bits & -bits
// 		putQueen(n,row+1,(col|p),((pia|p)<<1),((na|p)>>1))
// 		bits &= bits-1
// 	}
// 	return
// }

// @lc code=end

