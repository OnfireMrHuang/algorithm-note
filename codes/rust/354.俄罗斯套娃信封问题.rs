/*
 * @lc app=leetcode.cn id=354 lang=rust
 *
 * [354] 俄罗斯套娃信封问题
 */

// @lc code=start
impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut envs = envelopes;
        // 对数组进行排序
        envs.sort_by(|x, y| {
            if x[0] == y[0] {
                return y[1].cmp(&x[1]);
            }
            x[0].cmp(&y[0])
        });
        let mut nums = Vec::new();
        for item in envs {
            nums.push(item[1])
        }
        Self::poker_lis(nums)
    }

    // 求数组的最长递增子序列
    // 假设F(i)表示第i个数时的最长递增子序列个数，那么F(i+1)则是F(i) + (N(i+1) > N(max) -> 1 else 0)
    fn dp_lis(nums: Vec<i32>) -> i32 {
        if nums.len() < 1 {
            return 0;
        }
        let mut dp: Vec<i32> = Vec::with_capacity(nums.len());
        dp.push(1);
        for i in 1..nums.len() {
            dp.push(1);
            for j in 0..i {
                if nums[i] > nums[j] {
                    let num_j = dp.get(j).unwrap() + 1;
                    if let Some(elem) = dp.get_mut(i) {
                        *elem = (*elem).max(num_j)
                    }
                }
            }
        }
        let mut max_num = 0;
        for dp_num in dp {
            max_num = max_num.max(dp_num);
        }
        max_num
    }

    // 优化：二分查找法，和一个纸牌游戏场景类似
    //https://leetcode.cn/problems/longest-increasing-subsequence/solution/dong-tai-gui-hua-she-ji-fang-fa-zhi-pai-you-xi-jia/
    fn poker_lis(nums: Vec<i32>) -> i32 {
        let mut piles = 0;
        let mut top = vec![0; nums.len()];
        for i in 0..nums.len() {
            // 要处理的扑克牌
            let poker = nums[i];
            let mut left = 0;
            let mut right = piles;
            // 二分查找位置
            while left < right {
                let mid = (left + right) / 2;
                if top[mid] >= poker {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            if left == piles {
                piles += 1;
            }
            // 把牌放到牌堆顶
            top[left] = poker;
        }
        // 最终牌堆数就是最长递增序列的长度
        piles as i32
    }
}
// @lc code=end
