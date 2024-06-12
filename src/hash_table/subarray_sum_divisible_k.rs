use std::collections::HashMap;
// https://leetcode.com/problems/subarray-sums-divisible-by-k/?envType=daily-question&envId=2024-06-09
// #prefixsum #hashmap
pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
    let mut ans = 0;
    let mut hp: HashMap<i32, i32> = HashMap::new();
    let mut prefix_mod = 0;
    hp.insert(0, 1);
    for i in nums {
        prefix_mod = (prefix_mod + i % k + k) % k;
        if let Some(index) = hp.get(&(prefix_mod)) {
            ans += index;
        }
        hp.entry(prefix_mod).and_modify(|e| *e += 1).or_insert(1);
    }

    ans
}
