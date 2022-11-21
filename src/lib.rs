pub mod bits;
pub mod collections;

#[cfg(test)]
mod tests {
    use super::*;

    use bits::fid::FID;
    use bits::fid::NaiveFID;

    use string::trie::Trie;
    use string::trie::NaiveTrie;

    #[test]
    fn it_works_naive_bitvector() {
        let mut fid = NaiveFID::new(4 * 1024 * 1024);
        fid.set(3 * 1024 * 1024, true);
        assert!(fid.access(3 * 1024 * 1024));
    }

    #[test]
    fn it_works_naive_trie() {
        let mut trie = NaiveTrie::new();
        trie.append("the");
        trie.append("they");
        trie.append("their");
        trie.append("them");
        trie.append("theirs");
        trie.append("this");
        trie.append("that");
        assert_eq!("the", trie.prefix("theorem"));
    }
}
