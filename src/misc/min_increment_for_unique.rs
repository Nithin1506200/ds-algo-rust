pub fn min_increment_for_unique(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let max = *nums.iter().max().unwrap_or(&0);
    let mut ans = 0;
    let mut vec: Vec<i32> = vec![0; n + (max + 1) as usize];

    for i in nums {
        vec[i as usize] += 1
    }
    for i in 0..vec.len() - 1 {
        let i = vec[i as usize];
        if i <= 1 {
            continue;
        }
        let duplicate = i - 1;
        vec[i as usize + 1] += duplicate;
        ans += duplicate;
    }
    ans
}
