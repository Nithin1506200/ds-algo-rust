// https://leetcode.com/problems/replace-words/description/?envType=daily-question&envId=2024-06-07

// In English, we have a concept called root, which can be followed by some other word to form another longer word - let's call this word derivative. For example, when the root "help" is followed by the word "ful", we can form a derivative "helpful".

// Given a dictionary consisting of many roots and a sentence consisting of words separated by spaces, replace all the derivatives in the sentence with the root forming it. If a derivative can be replaced by more than one root, replace it with the root that has the shortest length.

// Return the sentence after the replacement.

// Example 1:

// Input: dictionary = ["cat","bat","rat"], sentence = "the cattle was rattled by the battery"
// Output: "the cat was rat by the bat"

// Example 2:

// Input: dictionary = ["a","b","c"], sentence = "aadsfasf absbs bbab cadsfafs"
// Output: "a a b c"
use std::collections::HashSet;
pub fn replace_words_dict(dictionary: Vec<String>, sentence: String) -> String {
    let mut hash_set = HashSet::new();
    let (mut min_len, mut max_len) = (usize::MAX, 0);
    for i in dictionary {
        hash_set.insert(i.clone());
        min_len = min_len.min(i.len());
        max_len = max_len.max(i.len());
    }

    fn get_root(
        word: String,
        hash_set: &HashSet<String>,
        min_len: usize,
        max_len: usize,
    ) -> String {
        let _ = max_len;
        let len = word.len();
        if len < min_len {
            return word;
        }

        for i in 0..len {
            println!("------------ xxx xxxxxx {:?}  {:?}", i, len);
            let key = &word[0..i].to_string();
            if hash_set.contains(key) {
                return key.clone();
            }
        }
        word
    }
    let mut ans: Vec<String> = Vec::new();
    for str in sentence.split(" ").into_iter() {
        ans.push(get_root(str.to_string(), &hash_set, min_len, max_len));
    }

    ans.join(" ")
}

#[cfg(test)]
mod test {
    use self::super::*;
    #[test]
    fn replace_words_dict_test() {
        assert_eq!(
            replace_words_dict(
                vec!["cat".into(), "bat".into(), "rat".into()],
                "the cattle was rattled jjjjj by the battery".into()
            ),
            "the cat was rat by the bat"
        );
    }
}
