// https://leetcode.com/problems/longest-common-subsequence/description/
// 1143. Longest Common Subsequence
// Given two strings text1 and text2, return the length of their longest common subsequence. If there is no common subsequence, return 0.
// A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.
//     For example, "ace" is a subsequence of "abcde".
// A common subsequence of two strings is a subsequence that is common to both strings.

fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let m = text1.len();
    let n = text2.len();
    let text1 = text1.as_bytes();
    let text2 = text2.as_bytes();
    // Create DP table initialized with zeros
    let mut dp = vec![vec![0; n + 1]; m + 1];
    // Fill DP table
    for i in 1..=m {
        for j in 1..=n {
            if text1[i - 1] == text2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
    dp[m][n]
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(longest_common_subsequence("abcde".into(), "ace".into()), 3);
        assert_eq!(longest_common_subsequence("abc".into(), "abc".into()), 3);
        assert_eq!(longest_common_subsequence("abc".into(), "def".into()), 0);
    }
}
