//https://www.youtube.com/watch?v=mLfjzJsN8us&list=PLgUwDviBIf0qUlt5H_kiKYaNSqJ81PMMY&index=3
//https://leetcode.com/problems/climbing-stairs/description/

use std::collections::HashMap;

/*
Time Complexity: O(2^n), as we are making two recursive calls for each stair.
Auxiliary Space: O(n), as we are using recursive stack space. */
fn climb_stairs(n: i32) -> i32 {
    if n == 0 || n == 1 {
        1
    } else {
        climb_stairs(n - 1) + climb_stairs(n - 2)
    }
}
//Using Top-Down DP (Recursion) – O(n) Time and O(n) Space
fn climb_stairs_(n: i32) -> i32 {
    let mut memo = HashMap::new();

    fn memoized_climb_stairs(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if n == 0 || n == 1 {
            return 1;
        }
        match memo.get(&n) {
            Some(i) => *i,
            None => {
                let val = memoized_climb_stairs(n - 1, memo) + memoized_climb_stairs(n - 2, memo);
                memo.insert(n, val);
                val
            }
        }
    }

    // if the result for this subproblem is
    // already computed then return it

    memoized_climb_stairs(n, &mut memo)
}
#[test]
fn test() {
    assert_eq!(climb_stairs(2), 2);
    assert_eq!(climb_stairs(3), 3);

    assert_eq!(climb_stairs_(2), 2);
    assert_eq!(climb_stairs_(3), 3);
}
