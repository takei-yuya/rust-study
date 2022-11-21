pub mod naive_trie;
pub use naive_trie::NaiveTrie;

pub trait Trie {
    fn contains(&self, s: &str) -> bool;
    fn prefix<'a>(&self, s:&'a str) -> &'a str;
}
