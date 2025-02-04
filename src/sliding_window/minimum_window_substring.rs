// https://neetcode.io/problems/minimum-window-with-characters
// https://leetcode.com/problems/minimum-window-substring/description/

use std::collections::HashMap;

pub fn min_window(s: String, t: String) -> String {
    if t == "" {
        return "".into();
    }
    let mut count_t: HashMap<char, i32> = HashMap::new();
    let mut window_t: HashMap<char, i32> = HashMap::new();

    for i in 0..t.len() {
        *count_t.entry(t.chars().nth(i).unwrap()).or_insert(0) += 1;
    }
    let (mut have, mut left_ptr, mut res_len, need) = (0, 0, i32::MAX, count_t.len());
    let mut res: (i32, i32) = (-1, -1);

    for right_ptr in 0..s.len() {
        let c = s.chars().nth(right_ptr).unwrap();
        *window_t.entry(c).or_insert(0) += 1;
        if count_t.get(&c).is_some() && window_t.get(&c) == count_t.get(&c) {
            have += 1;
        };
        while have == need {
            if ((right_ptr - left_ptr + 1) as i32) < res_len {
                res_len = (right_ptr - left_ptr + 1) as i32;
                res = (left_ptr as i32, right_ptr as i32);
            }
            *window_t
                .entry(s.chars().nth(left_ptr).unwrap())
                .or_insert(0) -= 1;
            if count_t.get(&s.chars().nth(left_ptr).unwrap()).is_some()
                && window_t.get(&s.chars().nth(left_ptr).unwrap())
                    < count_t.get(&s.chars().nth(left_ptr).unwrap())
            {
                have -= 1;
            }
            left_ptr += 1;
        }
    }

    if res_len == i32::MAX && res.0 > -1 && res.1 > -1 {
        "".into()
    } else {
        s.get(res.0 as usize..(res.1 + 1) as usize).unwrap().into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        for (s, t, ans) in vec![
            ("ADOBECODEBANC", "ABC", "BANC"),
            ("a", "a", "a"),
            ("a", "aa", ""),
        ] {
            assert_eq!(min_window(s.into(), t.into()), String::from(ans))
        }
    }
}
