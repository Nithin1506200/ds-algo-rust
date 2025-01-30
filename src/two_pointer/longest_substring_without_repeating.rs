use std::collections::HashMap;
//Longest Substring Without Repeating Characters

// https://leetcode.com/problems/longest-substring-without-repeating-characters/

// Example 1:

// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3.
// Example 2:

// Input: s = "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.
// Example 3:

// Input: s = "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3.
// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut ans: i32 = 0;
    let mut hp: HashMap<char, i32> = HashMap::new();
    let mut left_ptr = 0;
    for (i, chr) in s.chars().enumerate() {
        if let Some(index) = hp.get(&chr) {
            left_ptr = (index + 1).max(left_ptr); // left ptr should be incremental
        }
        hp.insert(chr, i as i32);
        ans = ans.max(i as i32 - left_ptr + 1)
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(length_of_longest_substring(String::from("abcabcbb")), 3);
        assert_eq!(length_of_longest_substring(String::from("bbbbb")), 1);
        assert_eq!(length_of_longest_substring(String::from("pwwkew")), 3);
    }
}
