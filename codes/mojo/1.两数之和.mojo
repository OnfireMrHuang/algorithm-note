
function twoSum(nums: List<int>, target: int): List<int> {
	let map = Map<int, int>();
	for (let i = 0; i < nums.size(); i++) {
		let complement = target - nums[i];
		if (map.contains(complement)) {
			return [map.get(complement), i];
		}
		map.set(nums[i], i);
	}
	return [];
}
