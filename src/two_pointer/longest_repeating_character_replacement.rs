use std::{collections::HashMap, usize};

// https://neetcode.io/problems/longest-repeating-substring-with-replacement
// https://leetcode.com/problems/longest-repeating-character-replacement/description/
// Longest Repeating Character Replacement
/*
You are given a string s and an integer k. You can choose any character of the string and change it to any other uppercase English character. You can perform this operation at most k times.

Return the length of the longest substring containing the same letter you can get after performing the above operations.



Example 1:

Input: s = "ABAB", k = 2
Output: 4
Explanation: Replace the two 'A's with two 'B's or vice versa.

Example 2:

Input: s = "AABABBA", k = 1
Output: 4
Explanation: Replace the one 'A' in the middle with 'B' and form "AABBBBA".
The substring "BBBB" has the longest repeating letters, which is 4.
There may exists other ways to achieve this answer too.
 */
pub fn character_replacement(s: String, k: i32) -> i32 {
    let (mut left_indx, mut ans, mut temp_max) = (0, 0, 0);
    let mut hash: HashMap<char, usize> = HashMap::new();
    let s: Vec<char> = s.chars().collect();
    for right in 0..s.len() {
        *hash.entry(s[right]).or_insert(0) += 1;
        temp_max = temp_max.max(*hash.get(&s[right]).unwrap());
        while (right - left_indx + 1) - temp_max > k as usize {
            *hash.entry(s[left_indx]).or_insert(0) -= 1;
            left_indx += 1;
        }
        ans = ans.max(right - left_indx + 1);
    }
    ans as i32
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        vec![
            (String::from("ABAB"), 2, 4),
            (String::from("AABABBA"), 1, 4),
        ]
        .iter()
        .for_each(move |(s, k, output)| assert_eq!(character_replacement(s.clone(), *k), *output));
    }
}
