// https://leetcode.com/problems/palindromic-substrings/
fn get_palindrome(s: &String, l_pointer: i32, r_pointer: i32) -> i32 {
    let mut r_pointer = r_pointer;
    let mut l_pointer = l_pointer;
    let mut count = 0;
    while l_pointer >= 0 && r_pointer < s.len() as i32 {
        if s.chars().nth(r_pointer as usize) != s.chars().nth(l_pointer as usize) {
            break;
        }
        count += 1;
        r_pointer += 1;
        l_pointer -= 1;
    }
    count
}
pub fn count_substrings(s: String) -> i32 {
    let mut count: i32 = 0;
    for i in 0..s.len() {
        count += get_palindrome(&s, i as i32, i as i32);
        count += get_palindrome(&s, i as i32, i as i32 + 1);
    }
    count
}
