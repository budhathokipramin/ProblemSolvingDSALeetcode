// Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.

// Note that you must do this in-place without making a copy of the array.

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut last_non_zero_found_at = 0;
    let n = nums.len();
    
    // If the current element is not 0, then we need to
    // append it just in front of the last non zero element we found.
    for i in 0..n {
        if nums[i] != 0 {
            nums.swap(last_non_zero_found_at, i);
            last_non_zero_found_at += 1;
        }
    }

    }
}
