use super::Trie;

use std::collections::HashMap;


pub struct NaiveTrie {
    children: HashMap<char, Box<NaiveTrie>>,
    is_leaf: bool,
}

impl NaiveTrie {
    pub fn new() -> Self {
        let children = HashMap::new();
        NaiveTrie {
            children,
            is_leaf: false,
        }
    }

    pub fn append(&mut self, s: &str) -> bool {
        let mut node = self;
        for c in s.chars() {
            let entry = node.children.entry(c);
            node = entry.or_insert(Box::new(NaiveTrie::new()));
        }
        let is_new = !node.is_leaf;
        node.is_leaf = true;
        is_new
    }

    pub fn size(&self) -> usize {
        1_usize + self.children.values().map(|node| node.size()).sum::<usize>() as usize
    }
}

impl Trie for NaiveTrie {
    fn contains(&self, s: &str) -> bool {
        let mut node = self;
        for c in s.chars() {
            if let Some(v) = node.children.get(&c) {
                node = v;
            } else {
                return false;
            }
        }
        node.is_leaf == true
    }

    fn prefix<'a>(&self, s:&'a str) -> &'a str {
        let mut len = 0;
        let mut node = self;
        for (i, c) in s.chars().enumerate() {
            if let Some(v) = node.children.get(&c) {
                node = v;
                if node.is_leaf {
                    len = i + 1;
                }
            } else {
                return &s[0..len];
            }
        }
        &s[0..len]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains() {
        let mut node = NaiveTrie::new();
        assert!(node.append("foo"));
        assert_eq!(4, node.size());
        assert!(!node.append("foo"));
        assert_eq!(4, node.size());
        assert!(node.append("bar"));
        assert_eq!(7, node.size());
        assert!(node.append("baz"));
        assert_eq!(8, node.size());
        assert!(node.append("foobar"));
        assert_eq!(11, node.size());
        assert!(node.append("あいうえお"));
        assert_eq!(16, node.size());

        assert!(node.contains("foo"));
        assert!(node.contains("bar"));
        assert!(node.contains("baz"));
        assert!(node.contains("foobar"));
        assert!(node.contains("あいうえお"));

        assert!(!node.contains("fo"));
        assert!(!node.contains("foob"));
        assert!(!node.contains("xxx"));
        assert!(!node.contains("あいうえおか"));
    }

    #[test]
    fn prefix() {
        let mut node = NaiveTrie::new();
        node.append("foo");
        node.append("bar");
        node.append("baz");
        node.append("foobar");
        node.append("あいうえお");

        assert_eq!("", node.prefix(""));
        assert_eq!("", node.prefix("f"));
        assert_eq!("", node.prefix("fo"));
        assert_eq!("foo", node.prefix("foo"));
        assert_eq!("foo", node.prefix("foob"));
        assert_eq!("foo", node.prefix("fooba"));
        assert_eq!("foobar", node.prefix("foobar"));
        assert_eq!("foobar", node.prefix("foobarbaz"));
    }
}
