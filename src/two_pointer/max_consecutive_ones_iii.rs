// https://leetcode.com/problems/max-consecutive-ones-iii/description/

pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut ans: i32 = 0;
    let mut left_ptr: i32 = 0;
    let mut temp_k = 0;
    for (right_ptr, ele) in nums.iter().enumerate() {
        if *ele == 0 {
            temp_k += 1;
            while temp_k > k {
                if *&nums[left_ptr as usize] == 0 {
                    temp_k -= 1;
                }
                left_ptr += 1;
            }
        }
        ans = ans.max(right_ptr as i32 - left_ptr + 1)
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2), 6);
        assert_eq!(
            longest_ones(
                vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
                3
            ),
            10
        );
    }
}
