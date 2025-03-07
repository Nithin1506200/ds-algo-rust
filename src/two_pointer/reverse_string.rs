// https://leetcode.com/problems/reverse-string/?envType=daily-question&envId=2024-06-02
pub fn reverse_string(s: &mut Vec<char>) {
    let len: usize = s.len();
    for i in 0..len / 2 {
        let temp = s[i];
        s[i] = s[len - 1 - i];
        s[len - 1 - i] = temp;
    }
}
#[cfg(test)]
mod test {

    #[test]
    fn test() {
        // let vec: Vec<char> = vec!["a".into(), "s", "k"];
        // assert_eq!(reverse_string(&mut vec), vec!["k", "s", "a"]);
    }
}
