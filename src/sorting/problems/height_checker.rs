// https://leetcode.com/problems/height-checker/description/?envType=daily-question&envId=2024-06-10

pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut sorted_vec = heights.clone();
    sorted_vec.sort();
    let mut ans = 0;
    for i in 0..heights.len() {
        if &sorted_vec[i] != &heights[i] {
            ans += 1;
        }
    }
    ans
}
