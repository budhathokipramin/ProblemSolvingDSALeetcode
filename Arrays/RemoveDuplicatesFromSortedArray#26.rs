// Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in nums.

// Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:

// Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially. The remaining elements of nums are not important as well as the size of nums.

// Return k.

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if(nums.len()==0){
            return 0;
        }

        let mut uniqueindex = 0;

        for i in 1..nums.len(){
            if(nums[i]!=nums[uniqueindex]){
                uniqueindex+=1;
                nums[uniqueindex]=nums[i];
            }
        }
        return (uniqueindex+1) as i32;
    }
}
