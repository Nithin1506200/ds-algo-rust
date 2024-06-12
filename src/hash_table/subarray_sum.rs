// https://leetcode.com/problems/continuous-subarray-sum/editorial/?envType=daily-question&envId=2024-06-08

// Given an integer array nums and an integer k, return true if nums has a good subarray or false otherwise.

// A good subarray is a subarray where:

//     its length is at least two, and
//     the sum of the elements of the subarray is a multiple of k.

// Note that:

//     A subarray is a contiguous part of the array.
//     An integer x is a multiple of k if there exists an integer n such that x = n * k. 0 is always a multiple of k.

use std::collections::HashMap;

/**
Let nnn be the number of elements in nums.

    Time complexity: O(n)

We iterate through the array exactly once. In each iteration, we perform a search operation in the hashmap that takes O(1)O(1)O(1) time. Therefore, the time complexity can be stated as O(n)

    Space complexity: O(n)

In each iteration, we insert a key-value pair in the hashmap. The space complexity is O(n)O(n)O(n) because the size of the hashmap is proportional to the size of the list after nnn iterations.


The sum of a subarray starting at index i+1i+1 and ending at jj is given by prefix[j]−prefix[i]prefix[j]−prefix[i].

If the sum of this subarray is divisible by kk:
(prefix[j]−prefix[i])%k=0(1)

Expressing in terms of divisor, dividend and remainder:

Let
```
prefix[j]=Q1⋅k+R1, prefix[i]=Q2⋅k+R2
```

Put these values in (1).
```
((Q1−Q2)⋅k+(R1−R2))%k=0
((Q1−Q2)⋅k+(R1−R2))%k=0

```

Since,
```
(Q1−Q2)⋅k(Q1−Q2)⋅k is divisible by kk, R1−R2 R1−R2 is also divisible by kk.
```
Both the remainders lie in the range [0,k)[0,k). So, R1−R2R1−R2 can be divisible by kk if:
R1−R2=0R1=R2


Putting the values R1=prefix[j]%kR1=prefix[j]%k, R2=prefix[i]%kR2=prefix[i]%k in (2).
prefix[j]%k=prefix[i]%k


Therefore, prefix sum up to index jj modulo kk should be equal to prefix sum up to index ii modulo kk.

#prefixsum #hashmap
*/

pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    let mut prefix_mod = 0;
    let mut mod_seen: HashMap<i32, i32> = HashMap::new();
    mod_seen.insert(0, -1); //think
                            //or
                            //     if nums.len() <2 {
                            //     return false
                            // }
    for (i, ele) in nums.into_iter().enumerate() {
        prefix_mod = (prefix_mod + ele) % k;

        if let Some(indx) = mod_seen.get(&prefix_mod) {
            if i as i32 - *indx > 1 {
                //to handle nums=[0] and k=1 atleast 2 digit
                return true;
            }
        } else {
            mod_seen.insert(prefix_mod, i as i32);
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(check_subarray_sum(vec![23, 2, 4, 6, 7], 6), true);
        assert_eq!(check_subarray_sum(vec![23, 2, 6, 4, 7], 6), true);
        assert_eq!(check_subarray_sum(vec![23, 2, 6, 4, 7], 13), false);
        assert_eq!(check_subarray_sum(vec![0], 1), false);
    }
}
