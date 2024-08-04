// Given an array A[ ] of positive integers of size N, where each value represents the number of chocolates in a packet. Each packet can have a variable number of chocolates. There are M students, the task is to distribute chocolate packets among M students such that :
// 1. Each student gets exactly one packet.
// 2. The difference between maximum number of chocolates given to a student and minimum number of chocolates given to a student is minimum.


impl Solution {
    fn find_min_diff(arr: &mut [i32], n: usize, m: usize) -> i32 {
        // Sort the array
        arr.sort();

        // Initialize the minimum difference to a large value
        let mut min_diff = i32::MAX;

        // Iterate over the array with a window size of m
        for i in 0..=n - m {
            // Calculate the difference between the maximum and minimum in the current window
            let diff = arr[i + m - 1] - arr[i];

            // Update the minimum difference if a smaller one is found
            if diff < min_diff {
                min_diff = diff;
            }
        }

        return min_diff
    }
}
