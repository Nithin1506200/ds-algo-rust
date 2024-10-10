// https://leetcode.com/problems/max-consecutive-ones-iii/

/**
 * Given a binary array nums and an integer k, return the maximum number of consecutive 1's in the array if you can flip at most k 0's.
 */

pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut l_ptr: usize = 0;
    let mut ans: usize = 0;
    let mut k_count: i32 = 0;
    for r_ptr in 0..nums.len() {
        if nums[r_ptr] == 0 {
            k_count += 1;
            while k_count > k {
                if nums[l_ptr] == 0 {
                    k_count -= 1;
                }
                l_ptr += 1;
            }
        }
        ans = ans.max(r_ptr - l_ptr + 1)
    }
    ans as i32
}

#[test]
fn test() {
    assert!(longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2) == 6);
    assert!(
        longest_ones(
            vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
            3
        ) == 10
    );
}
