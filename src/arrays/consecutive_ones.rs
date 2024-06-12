pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut temp_max = 0;
    for i in nums {
        if i == 1 {
            temp_max += 1;
            max = max.max(temp_max);
        } else {
            temp_max = 0;
        }
    }
    max
}
