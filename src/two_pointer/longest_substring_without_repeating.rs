use std::collections::HashMap;

// https://leetcode.com/problems/longest-substring-without-repeating-characters/
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
    fn test() {}
}
