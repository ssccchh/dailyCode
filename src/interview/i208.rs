use crate::Solution;

struct Node {
    son: [Option<Box<Node>>; 26],
    end: bool,
}

impl Node {
    fn new() -> Self {
        Node {
            son: [const { None }; 26],
            end: false,
        }
    }
}

struct Trie {
    root: Node,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Trie { root: Node::new() }
    }

    fn insert(&mut self, word: String) {
        let mut cur = &mut self.root;
        for c in word.as_bytes() {
            let c = (c - b'a') as usize;
            if cur.son[c].is_none() {
                cur.son[c] = Some(Box::new(Node::new()))
            }
            cur = cur.son[c].as_mut().unwrap();
        }
        cur.end = true;
    }

    fn find(&mut self, word: String) -> usize {
        let mut cur = &mut self.root;
        for c in word.as_bytes() {
            let c = (c - b'a') as usize;
            if cur.son[c].is_none() {
                return 0;
            }
            cur = cur.son[c].as_mut().unwrap();
        }
        if cur.end {
            2
        } else {
            1
        }
    }

    fn search(&mut self, word: String) -> bool {
        self.find(word) == 2
    }

    fn starts_with(&mut self, prefix: String) -> bool {
        self.find(prefix) != 0
    }
}

impl Solution {
    pub fn run208() {
        let mut trie = Trie::new();

        trie.insert(String::from("abc"));
        trie.search(String::from("abc"));
        trie.starts_with(String::from("a"));
    }
}
