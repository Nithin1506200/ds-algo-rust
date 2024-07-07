use std::collections::HashSet;
//https://leetcode.com/problems/minimum-number-of-k-consecutive-bit-flips/submissions/1298503797/?envType=daily-question&envId=2024-06-24
pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
    let mut current_flips = 0;
    let mut total_flips = 0;
    let mut hs: HashSet<i32> = HashSet::new();
    for i in 0..nums.len() as i32 {
        if hs.get(&(i - k)).is_some() {
            current_flips -= 1;
        }
        if (current_flips % 2) == nums[i as usize] {
            if i + k > nums.len() as i32 {
                return -1;
            }
            hs.insert(i);
            current_flips += 1;
            total_flips += 1;
        }
    }
    total_flips
}
