// Given an array of integers nums containing n + 1 integers where each integer is in the range [1, n] inclusive.
// There is only one repeated number in nums, return this repeated number.
// You must solve the problem without modifying the array nums and uses only constant extra space.
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // Phase 1: Finding the intersection point in the cycle
        let mut tortoise = nums[0];
        let mut hare = nums[0];

        loop {
            tortoise = nums[tortoise as usize];
            hare = nums[nums[hare as usize] as usize];
            if tortoise == hare {
                break;
            }
        }

        // Phase 2: Finding the entrance to the cycle
        let mut ptr1 = nums[0];
        let mut ptr2 = tortoise;
        while ptr1 != ptr2 {
            ptr1 = nums[ptr1 as usize];
            ptr2 = nums[ptr2 as usize];
        }

        ptr1 // or ptr2, as they are equal
    }
}
