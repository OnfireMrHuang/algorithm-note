use std::collections::HashMap;

struct NumArray {
	pre_sum_map: HashMap<i32, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
		let mut pre_sum_map = HashMap::new();
		let mut pre_sum = 0;
		for (i, num) in nums.iter().enumerate() {
			pre_sum += num;
			pre_sum_map.insert(i as i32, pre_sum);
		}
		Self {
			pre_sum_map,
		}
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
		self.pre_sum_map.get(&right).unwrap_or(&0) -  self.pre_sum_map.get(&(left - 1)).unwrap_or(&0)
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */