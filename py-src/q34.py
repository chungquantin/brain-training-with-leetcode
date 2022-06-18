from typing import List


class Solution:
    def searchRange(self, nums: List[int], target: int) -> List[int]:
        if not nums:
            return [-1, -1]
        return [self.bisect_left(nums, target), self.bisect_right(nums, target)]

    def bisect_left(self, nums, target):
        l, r = 0, len(nums) - 1
        while l < r:
            m = (l + r) // 2
            if nums[m] < target:
                l = m + 1
            else:
                r = m
        return l if nums[l] == target else -1

    def bisect_right(self, nums, target):
        l, r = 0, len(nums) - 1
        while l < r:
            m = (l + r) // 2 + 1
            if nums[m] > target:
                r = m - 1
            else:
                l = m
        return l if nums[l] == target else -1


# print(Solution().searchRange([2, 3, 3, 4], 4), [3, 3])
print(Solution().searchRange([5,7,7,8,8,10], 8), [3, 4])
