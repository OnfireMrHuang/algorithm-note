// 注意：go 代码由 chatGPT🤖 根据我的 java 代码翻译，旨在帮助不同背景的读者理解算法逻辑。
// 本代码已经通过力扣的测试用例，应该可直接成功提交。

func numTrees(n int) int {
	// 备忘录的值初始化为 0
	memo := make([][]int, n+1)
	for i := 0; i <= n; i++ {
		memo[i] = make([]int, n+1)
	}

	var count func(lo, hi int) int
	count = func(lo, hi int) int {
		if lo > hi {
			return 1
		}
		// 查备忘录
		if memo[lo][hi] != 0 {
			return memo[lo][hi]
		}

		res := 0
		for mid := lo; mid <= hi; mid++ {
			left := count(lo, mid-1)
			right := count(mid+1, hi)
			res += left * right
		}
		// 将结果存入备忘录
		memo[lo][hi] = res

		return res
	}

	return count(1, n)
}