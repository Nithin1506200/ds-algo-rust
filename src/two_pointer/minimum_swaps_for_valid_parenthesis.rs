//Minimum Number of Swaps to Make the String Balanced

// https://leetcode.com/problems/minimum-number-of-swaps-to-make-the-string-balanced/description/?envType=daily-question&envId=2024-10-08

pub fn min_swaps(s: String) -> i32 {
    let mut deque: Vec<char> = vec![];
    let mut unbalanced: i32 = 0;

    for i in s.chars() {
        if i == '[' {
            deque.push(i);
        } else {
            if !deque.is_empty() {
                deque.pop();
            } else {
                unbalanced += 1;
            }
        }
    }
    (unbalanced + 1) / 2
}

pub fn min_swaps_(s: String) -> i32 {
    let mut opening: i32 = 0;
    let mut unbalanced: i32 = 0;

    for i in s.chars() {
        if i == '[' {
            opening += 1;
        } else {
            if opening != 0 {
                opening -= 1;
            } else {
                unbalanced += 1;
            }
        }
    }
    (unbalanced + 1) / 2
}
#[test]
fn test() {
    let test_cases = [("][][".to_string(), 1), ("]]][[[".to_string(), 2)];
    for (s, ans) in test_cases {
        assert_eq!(min_swaps(s.clone()), ans);
        assert_eq!(min_swaps_(s), ans);
    }
}
