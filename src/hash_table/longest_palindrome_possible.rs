use std::collections::HashMap;

pub fn longest_palindrome(s: String) -> i32 {
    let mut ans = 0;
    let mut hs: HashMap<char, ()> = HashMap::new();
    for char in s.chars() {
        if hs.get(&char).is_some() {
            ans += 2;
            hs.remove(&char);
        } else {
            hs.insert(char, ());
        }
    }
    if !hs.is_empty() {
        ans += 1
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(longest_palindrome("abccccdd".into()), 7);
        assert_eq!(longest_palindrome("a".into()), 1);
    }
}
