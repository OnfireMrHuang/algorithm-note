impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&num) {
                if i - j <= k as usize {
                    return true;
                }
            }
            map.insert(num, i);
        }
        false
    }
}
