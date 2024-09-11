// https://en.wikipedia.org/wiki/Josephus_problem
// https://leetcode.com/problems/find-the-winner-of-the-circular-game/solutions/5437958/josephus-problem-recusion-1-line-vs-iteration-0ms-beats-100/?envType=daily-question&envId=2024-07-08

pub fn find_the_winner(n: i32, k: i32) -> i32 {
    if n == 1 {
        1
    } else {
        (find_the_winner(n - 1, k) + (k - 1)) % n + 1
    }
}
