// https://leetcode.com/problems/append-characters-to-string-to-make-subsequence/?envType=daily-question&envId=2024-06-03

pub fn append_characters(s: String, t: String) -> i32 {
    let mut ans = t.len() as i32;
    let mut t_iter = t.chars();
    let mut t_char = t_iter.next();
    for s_iter in s.chars() {
        if let Some(t_) = t_char {
            if t_ == s_iter {
                t_char = t_iter.next();
                ans -= 1;
            }
        } else {
            break;
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            append_characters(String::from("coaching"), String::from("coading")),
            4
        );
    }
}
