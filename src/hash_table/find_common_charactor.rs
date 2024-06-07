use std::collections::HashMap;

pub fn common_chars(words: Vec<String>) -> Vec<String> {
    let mut hp: HashMap<char, usize> = HashMap::new();

    for (i, word) in words.iter().enumerate() {
        if i == 0 {
            for e in word.chars() {
                hp.insert(e, hp.get(&e).map_or(1, |e| *e + 1));
            }
        } else {
            print!("{:?}", hp);
            let mut local_hp: HashMap<char, usize> = HashMap::new();
            for e in word.chars() {
                local_hp.insert(e, local_hp.get(&e).map_or(1, |e| *e + 1));
            }
            let keys: Vec<char> = hp.keys().map(|e| *e).collect();
            for key in keys {
                let min = hp
                    .get(&key)
                    .map_or(0, |e| *e)
                    .min(local_hp.get(&key).map_or(0, |e| *e));
                if min == 0 {
                    hp.remove(&key);
                } else {
                    hp.insert(key, min);
                }
            }
        }
    }
    let mut ans = vec![];
    for (key, val) in hp {
        for _ in 0..val {
            ans.push(String::from(key));
        }
    }
    ans
}

pub fn common_chars2(words: Vec<String>) -> Vec<String> {
    let mut common_chars: [usize; 26] = [0; 26];

    // Initialize common_chars with counts from the first word
    for c in words[0].chars() {
        common_chars[(c as u8 - b'a') as usize] += 1;
    }

    // Iterate through remaining words
    for word in words.iter().skip(1) {
        let mut word_chars: [usize; 26] = [0; 26];
        for c in word.chars() {
            word_chars[(c as u8 - b'a') as usize] += 1;
        }
        // Update common_chars by taking the minimum of existing count and count in current word
        for i in 0..26 {
            common_chars[i] = common_chars[i].min(word_chars[i]);
        }
    }

    // Construct the result array
    let mut result = Vec::new();
    for i in 0..26 {
        for _ in 0..common_chars[i] {
            result.push(((b'a' + i as u8) as char).to_string());
        }
    }

    result
}

#[cfg(test)]

mod test {
    use super::*;
    #[test]
    fn test() {
        println!(
            "{:?}",
            common_chars(vec!["bella".into(), "label".into(), "roller".into()])
        );
    }
}
