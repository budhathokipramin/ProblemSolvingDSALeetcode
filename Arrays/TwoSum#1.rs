// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

// You may assume that each input would have exactly one solution, and you may not use the same element twice.

//You can return the answer in any order.
//

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Create a vector of tuples (value, index)
        let mut indexed_nums: Vec<(i32, usize)> = nums.into_iter()
            .enumerate()
            .map(|(index, value)| (value, index))
            .collect();
        
        // Sort the vector by value
        indexed_nums.sort_by_key(|&(value, _)| value);
        
        let mut left = 0;
        let mut right = indexed_nums.len() - 1;

        while left < right {
            let left_value = indexed_nums[left].0;
            let right_value = indexed_nums[right].0;
            let sum = left_value + right_value;

            if sum == target {
                // Return original indices of the two numbers
                return vec![indexed_nums[left].1 as i32, indexed_nums[right].1 as i32];
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }

        vec![] // Return an empty vector if no solution is found
    }
}

