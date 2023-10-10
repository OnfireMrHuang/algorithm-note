/*
 * @lc app=leetcode.cn id=875 lang=rust
 *
 * [875] 爱吃香蕉的珂珂
 */

// @lc code=start
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut l = 1;
        let mut r = i32::MIN;
        // 可以先遍历一下所有堆, speed一定是在最小值到最大值之前的速度值
        for pile in piles.iter() {
            if pile.to_owned() > r {
                r = pile.to_owned()
            }
        }
        while l < r {
            let mid = l + r >> 1;
            if Self::check_speed(&piles, mid, h) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        r
    }

    fn check_speed(piles: &Vec<i32>, speed: i32, h: i32) -> bool {
        let mut ans = 0;
        for pile in piles {
            ans += (pile.to_owned() as f64 / speed as f64).ceil() as i32;
        }
        // 如果耗时比h要小，说明speed速度大于或等于最小速度, 否则就是speed比最小速度慢
        ans <= h
    }
}
// @lc code=end
