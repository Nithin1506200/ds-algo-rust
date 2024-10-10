//https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/editorial/?envType=daily-question&envId=2024-10-09

// time O(n) space=O(1)
pub fn min_add_to_make_valid(s: String) -> i32 {
    let mut closing_brac: i32 = 0;
    let mut opening_brac: i32 = 0;

    for i in s.chars() {
        if i == '(' {
            opening_brac += 1;
        } else {
            if opening_brac > 0 {
                opening_brac -= 1;
            } else {
                closing_brac += 1;
            }
        }
    }
    closing_brac + opening_brac
}

// time O(n) space=O(N)
pub fn min_add_to_make_valid_(s: String) -> i32 {
    let mut ans: Vec<char> = vec![];
    let opening = '(';

    for i in s.chars() {
        if i == opening {
            ans.push(opening);
        } else {
            if ans.ends_with(&[opening]) {
                ans.pop();
            } else {
                ans.push(i);
            }
        }
    }
    ans.len() as i32
}
#[test]
fn test() {
    assert_eq!(min_add_to_make_valid("())".to_string()), 1);
    assert_eq!(min_add_to_make_valid("(((".to_string()), 3);
    assert_eq!(min_add_to_make_valid("()))((".to_string()), 4);
}
