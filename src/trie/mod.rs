mod replace;

#[derive(Debug)]
pub struct Trie {
    dict: [Option<Box<Trie>>; 26],
    ends_here: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            dict: [const { None }; 26],
            ends_here: false,
        }
    }

    fn insert(&mut self, val: &str) {
        if val.is_empty() {
            return self.ends_here = true;
        }
        self.dict[(val.as_bytes()[0] - b'a') as usize]
            .get_or_insert_with(|| Box::new(Self::new()))
            .insert(&val[1..])
    }

    fn exists(&self, bc: u8) -> Option<&Self> {
        self.dict[(bc - b'a') as usize].as_ref().map(|v| &**v)
    }
}
// https://leetcode.com/problems/replace-words/solutions/5272009/trie-learn-to-use-the-right-data-structure-best-solution
