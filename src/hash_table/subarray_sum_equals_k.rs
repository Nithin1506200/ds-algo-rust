use std::collections::HashMap;
// #prefixsum #hashmap
pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut ans = 0;
    let mut prefix_sum = 0;
    let mut hp: HashMap<i32, i32> = HashMap::new();
    hp.insert(0, 1);
    for i in nums {
        prefix_sum += i;
        if let Some(count) = hp.get(&(prefix_sum - k)) {
            ans += count;
        }
        hp.insert(prefix_sum, hp.get(&prefix_sum).unwrap_or(&0) + 1);
        // m.entry(ps).and_modify(|x| *x += 1).or_insert(1);
    }
    ans
}
