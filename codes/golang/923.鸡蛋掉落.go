/*
 * @lc app=leetcode.cn id=923 lang=golang
 *
 * [923] 鸡蛋掉落
 *
 * https://leetcode-cn.com/problems/3sum-with-multiplicity/description/
 *
 * algorithms
 * Medium (25.77%)
 * Total Accepted:    473
 * Total Submissions: 1.8K
 * Testcase Example:  '[1,1,2,2,3,3,4,4,5,5]\n8'
 *
 * 给定一个整数数组 A，以及一个整数 target 作为目标值，返回满足 i < j < k 且 A[i] + A[j] + A[k] == target
 * 的元组 i, j, k 的数量。
 *
 * 由于结果会非常大，请返回 结果除以 10^9 + 7 的余数。
 *
 *
 *
 * 示例 1：
 *
 * 输入：A = [1,1,2,2,3,3,4,4,5,5], target = 8
 * 输出：20
 * 解释：
 * 按值枚举（A[i]，A[j]，A[k]）：
 * (1, 2, 5) 出现 8 次；
 * (1, 3, 4) 出现 8 次；
 * (2, 2, 4) 出现 2 次；
 * (2, 3, 3) 出现 2 次。
 *
 *
 * 示例 2：
 *
 * 输入：A = [1,1,2,2,2,2], target = 5
 * 输出：12
 * 解释：
 * A[i] = 1，A[j] = A[k] = 2 出现 12 次：
 * 我们从 [1,1] 中选择一个 1，有 2 种情况，
 * 从 [2,2,2,2] 中选出两个 2，有 6 种情况。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 3 <= A.length <= 3000 -- 数组的长度不会小于3，同时不会大于3000
 * 0 <= A[i] <= 100 -- 数组中的数值大小不会超过100
 * 0 <= target <= 300 -- 目标值不会超过300
 *
 *
 */

//得事先假设这是一个排序好的数组

const mod = 1e9 + 7

func threeSumMulti(A []int, target int) int {
	//做一个数组来做数值的计数映射
	count := [101]int{}
	for _, a := range A {
		count[a]++
	}

	res := 0

	//对数值进行遍历
	for Ai := 0; Ai <= 100; Ai++ {
		//因为i<=j<=k，所以设置以下表达式
		for Aj := Ai; Aj <= 100; Aj++ {
			Ak := target - Ai - Aj
			if Ak < 0 || Ak > 100 {
				continue
			}
			switch {
			case Ai == Aj && Aj == Ak:
				//通过值索引对应的值来表达会出现的次数
				res += count[Ai] * (count[Ai] - 1) * (count[Ai] - 2) / 6
			case Ai == Aj && Ak > Aj:
				res += count[Ai] * (count[Ai] - 1) / 2 * count[Ak]
			case Aj > Ai && Ak == Aj:
				res += count[Ai] * count[Aj] * (count[Aj] - 1) / 2
			case Aj > Ai && Ak > Aj:
				res += count[Ai] * count[Aj] * count[Ak]

			}
		}
	}
	return res % mod
}
