impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut p = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[p] = nums[i];
                p += 1;
            }
        }
        for i in p..nums.len() {
            nums[i] = 0;
        }
    }
}
