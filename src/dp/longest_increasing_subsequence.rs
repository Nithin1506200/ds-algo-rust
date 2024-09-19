/**
https://leetcode.com/problems/longest-increasing-subsequence/description/
Given an array arr[] of size N, the task is to find the length of the Longest Increasing Subsequence (LIS) i.e., the longest possible subsequence in which the elements of the subsequence are sorted in increasing order.

    Input: arr[] = {3, 10, 2, 1, 20}
    Output: 3
    Explanation: The longest increasing subsequence is 3, 10, 20

    Input: arr[] = {30, 20, 10}
    Output:1
    Explanation: The longest increasing subsequences are {30}, {20} and (10)

    Input: arr[] = {2, 2, 2}
    Output: 1
    Explanation: We consider only strictly increasing.

    Input: arr[] = {10, 20, 35, 80}
    Output: 4
    Explanation: The whole array is sorted

*/
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    // each element represents the length of lis ending at that index
    let mut arr = vec![1; nums.len()];

    for i in 0..(nums.len()) {
        for j in 0..(i) {
            if nums[i] > nums[j] {
                if arr[j] + 1 > arr[i] {
                    arr[i] = arr[j] + 1
                }
            }
        }
    }
    **arr.iter().max().get_or_insert(&0)
}
