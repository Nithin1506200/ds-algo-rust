// https://leetcode.com/problems/score-of-a-string/?envType=daily-question&envId=2024-06-01

// You are given a string s. The score of a string is defined as the sum of the absolute difference between the ASCII values of adjacent characters.You are given a string s. The score of a string is defined as the sum of the absolute difference between the ASCII values of adjacent characters.

pub fn score_of_string(s: String) -> i32 {
    let mut score = 0;
    let char: Vec<char> = s.chars().collect(); // might create extra memory
    for i in 0..s.len() - 1 {
        let left = char[i] as u8;
        let right = char[i + 1] as u8;
        score += (right).abs_diff(left) as i32;
    }

    score
}

pub fn score_of_string2(s: String) -> i32 {
    let mut score = 0;
    // might create extra memory
    let mut chars = s.chars(); // iter wont work
    for i in 0..s.len() - 1 {
        let left = chars.nth(i).unwrap() as i32;
        let right = chars.nth(i + 1).unwrap() as i32;
        score += (right - left).abs() as i32;
    }

    score
}

pub fn score_of_string3(s: String) -> i32 {
    let mut score = 0;
    // might create extra memory
    // iter wont work
    for i in 0..s.len() - 1 {
        let left = s.chars().nth(i).unwrap() as i32;
        let right = s.chars().nth(i + 1).unwrap() as i32;
        score += (right - left).abs() as i32;
    }

    score
}
