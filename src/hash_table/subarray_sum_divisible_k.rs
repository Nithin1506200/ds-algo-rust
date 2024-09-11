use std::collections::HashMap;
// https://leetcode.com/problems/subarray-sums-divisible-by-k/?envType=daily-question&envId=2024-06-09
// #prefixsum #hashmap
/**
 * Intuition
The problem is based on the concept of using prefix sums to compute the total number of subarrays that are divisible by k. A prefix sum array for nums is another array prefixSum of the same size as nums, such that the value of prefixSum[i] is the sum of all elements of the nums array from index 0 to index i, i.e., nums[0] + nums[1] + nums[2] + . . . + nums[i].

The sum of the subarray i + 1 to j (inclusive) is computed by prefixSum[j] - prefixSum[i]. Using this, we can count the number of pairs that exist for every pair (i, j) where i < j and (prefixSum[j] - prefix[i]) % k = 0. There are n * (n - 1) / 2 pairs for an array of length n (pick any two from n). As a result, while this will provide the correct answer for every test case, it will take O(n
2
 ) time, indicating that the time limit has been exceeded (TLE).

The character % is the modulo operator.

Let's try to use the information with respect to the remainders of every prefix sum and try to optimize the above approach.

As stated previously, our task is to determine the number of pairs (i, j) where i < j and (prefixSum[j] - prefix[i]) % k = 0. This equality can only be true if prefixSum[i] % k = prefixSum[j] % k. We will demonstrate this property.

We can express any number as number = divisor × quotient + remainder. For example, 13 when divided by 3 can be written as 13 = 3 * 4 + 1. So we can express:
a) prefixSum[i] as prefixSum[i] = A * k + R0 where A is the quotient and R0 is the remainder when divided by k.
b) Similarly, prefixSum[j] = B * k + R1 where B is the quotient and R1 is the remainder when divided by k.

We can write, prefixSum[j] - prefixSum[i] = k * (B - A) + (R1 - R0). The first term (k * (B - A)) is divisible by k, so for the entire expression to be divisible by k, R1 - R0 must also be divisible by k. This gives us an equation R1 - R0 = C * k, where C is some integer. Rearranging it yields R1 = C * k + R0. Because the values of R0 and R1 will be between 0 and k - 1, R1 cannot be greater than k. So the only possible value for C is 0, leading to R0 = R1, which proves the above property. If C > 0, then the RHS would be at least k, but as stated the LHS (R1) is between 0 and k - 1.

Here are two visual examples showing the calculations:
 */
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
