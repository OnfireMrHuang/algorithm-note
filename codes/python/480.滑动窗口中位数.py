

import heapq

class DualHeap:
	def __init__(self, k):
		# self.small是一个大根堆，保存较小的一半元素
		self.small = list()
		# self.large是一个小根堆，保存较大的一半元素
		self.large = list()
		# 哈希表，保存延迟删除的元素，key为元素，value为需要删除的次数
		self.delayed = {}
		self.k = k
		# small和large中元素的个数，需要扣除被延迟删除的元素
		self.small_size = 0
		self.large_size = 0

	def prune(self, heap):
		while heap:
			num = heap[0]
			if heap is self.small:
				num = -num
			if num in self.delayed:
				heapq.heappop(heap)
				self.delayed[num] -= 1
				if self.delayed[num] == 0:
					self.delayed.pop(num)
			else:
				break

	def make_balance(self):
		if self.small_size > self.large_size + 1:
			# small比large多2个元素
			heapq.heappush(self.large, -self.small[0])
			heapq.heappop(self.small)
			self.small_size -= 1
			self.large_size += 1
			# small堆顶元素被移除，需要进行 prune
			self.prune(self.small)
		elif self.small_size < self.large_size:
			# large比small多1个元素
			heapq.heappush(self.small, -self.large[0])
			heapq.heappop(self.large)
			self.small_size += 1
			self.large_size -= 1
			# large堆顶元素被移除，需要进行 prune
			self.prune(self.large)

	def insert(self, num):
		if not self.small or num <= -self.small[0]:
			heapq.heappush(self.small, -num)
			self.small_size += 1
		else:
			heapq.heappush(self.large, num)
			self.large_size += 1
		self.make_balance()

	def erase(self, num):
		self.delayed[num] = self.delayed.get(num, 0) + 1
		if num <= -self.small[0]:
			self.small_size -= 1
			if num == -self.small[0]:
				self.prune(self.small)
		else:
			self.large_size -= 1
			if num == self.large[0]:
				self.prune(self.large)
		self.make_balance()

	def get_median(self):
		return float(-self.small[0]) if self.k & 1 else (-self.small[0] + self.large[0]) / 2
	
class Solution:
	def medianSlidingWindow(self, nums, k):
		dual_heap = DualHeap(k)
		for num in nums[:k]:
			dual_heap.insert(num)
		ans = [dual_heap.get_median()]
		for i in range(k, len(nums)):
			dual_heap.insert(nums[i])
			dual_heap.erase(nums[i - k])
			ans.append(dual_heap.get_median())
		return ans