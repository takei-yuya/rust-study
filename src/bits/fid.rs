pub mod naive_fid;
pub use naive_fid::NaiveFID;

/// Fully Indexable Dictionary
/// rank操作およびselect操作が可能なビットベクトル
///
/// # Examples
///
/// ```
/// use rust_study::bits::fid::*;
/// let mut fid = NaiveFID::from_bool_vec(&vec![true, true, false, true, false, false, true, false]);
/// assert_eq!(8, fid.len());
///
/// // get/set はビットベクトルの i番目(0-based)にアクセスをします
/// assert_eq!(true, fid.get(3));
/// fid.set(3, false);
/// assert_eq!(false, fid.get(3));
/// fid.set(3, true);
///
/// // rankN はビットベクトルの [0, i) の中の N の数を数えます。
/// assert_eq!(1, fid.rank0(4));
/// assert_eq!(3, fid.rank1(4));
///
/// // selectN は i番目(0-based)の N の位置を返します。
/// assert_eq!(5, fid.select0(2));
/// assert_eq!(3, fid.select1(2));
/// ```
pub trait FID {
    /// 長さ `n` ですべてのビットが 0 のビットベクトルを作成します。
    fn new(n: usize) -> Self;

    /// Booleanベクトル `vec` から新しいビットベクトルを作成します。
    /// `false` は 0 、 `true` は 1 としてビットベクトルを構築します。
    fn from_bool_vec(vec: &Vec<bool>) -> Self;

    /// ビットベクトルの `i` 番目(0-based)のビットにアクセスします。
    ///
    /// # Panics
    ///
    /// `i` が境界の外にあるときパニックします。
    fn get(&self, i: usize) -> bool;

    /// ビットベクトルの `i` 番目(0-based)のビットを変更します。
    /// `bit` が `false` のとき 0 、 `true` のときは 1 として変更します。
    ///
    /// # Panics
    ///
    /// `i` が境界の外にあるときパニックします。
    fn set(&mut self, i: usize, bit: bool) -> ();

    /// ビットベクトルの長さを返します。
    fn len(&self) -> usize;

    /// ビットベクトルの `i` 番目(0-based)のビットにアクセスします。
    /// `get` と同じです。
    ///
    /// # Panics
    ///
    /// `i` が境界の外にあるときパニックします。
    fn access(&self, i: usize) -> bool;

    /// ビットベクトルの [0, `i`) の中の `0` の個数を数えます。
    ///
    /// # Panics
    ///
    /// `i` が境界の外にあるときパニックします。
    fn rank0(&self, i: usize) -> usize {
        i - self.rank1(i)
    }

    /// ビットベクトルの [0, `i`) の中の `1` の個数を数えます。
    ///
    /// # Panics
    ///
    /// `i` が境界の外にあるときパニックします。
    fn rank1(&self, i: usize) -> usize;

    /// `i` 番目(0-based)の `0` の位置を返します。
    /// `0` の個数が `i` 以上の場合、ビットベクトルの長さを返します。
    fn select0(&self, i: usize) -> usize {
        let mut beg = 0;
        let mut end = self.len();
        if self.rank0(end) <= i {
            return end;
        }
        loop {
            let p = (beg + end) / 2;
            let rank = self.rank0(p);

            if beg == end || beg + 1 == end {
                return beg;
            } else if i < rank {
                end = p;
            } else if rank <= i {
                beg = p;
            }
        }
    }

    /// `i` 番目(0-based)の `1` の位置を返します。
    /// `1` の個数が `i` 以上の場合、ビットベクトルの長さを返します。
    fn select1(&self, i: usize) -> usize {
        let mut beg = 0;
        let mut end = self.len();
        if self.rank1(end) <= i {
            return end;
        }
        loop {
            let p = (beg + end) / 2;
            let rank = self.rank1(p);

            if beg == end || beg + 1 == end {
                return beg;
            } else if i < rank {
                end = p;
            } else if rank <= i {
                beg = p;
            }
        }
    }
}

#[cfg(test)]
#[generic_tests::define]
mod tests {
    use super::*;
    use std::cmp::PartialEq;
    use std::fmt::Debug;
    use std::ops::Not;
    use rand::Rng;

    #[instantiate_tests(<NaiveFID>)]
    mod naive {}

    #[test]
    fn set_get<T: FID>() {
        let len = 1000;
        let mut rng = rand::thread_rng();

        let mut bv = vec![false; len];
        for i in 0..len {
            bv[i] = rng.gen();
        }
        let mut fid = T::from_bool_vec(&bv);
        // check overwrite
        for i in 0..len {
            bv[i] = rng.gen();
            fid.set(i, bv[i]);
        }

        for i in 0..len {
            assert_eq!(bv[i], fid.get(i));
        }
    }

    #[test]
    fn from_bool_vec<T: FID + PartialEq + Debug>() {
        let len = 1000;
        let mut rng = rand::thread_rng();
        let mut bv = vec![false; len];
        let mut expected = T::new(len);
        for i in 0..len {
            bv[i] = rng.gen();
            expected.set(i, bv[i]);
        }
        let fid = T::from_bool_vec(&bv);
        assert_eq!(expected, fid);
    }

    #[test]
    fn rank<T: FID>() {
        let mut rng = rand::thread_rng();
        let len = 1000;

        let mut bv = vec![false; len];
        for i in 0..len {
            bv[i] = rng.gen();
        }
        let mut fid = T::from_bool_vec(&bv);
        // check if set/unset updates offsets correclty
        for i in 0..len {
            bv[i] = rng.gen();
            fid.set(i, bv[i]);
        }

        let mut rank0 = 0;
        let mut rank1 = 0;
        for i in 0..len {
            assert_eq!(rank0, fid.rank0(i));
            assert_eq!(rank1, fid.rank1(i));
            // rankN counts Ns in [0, i), so increment after check
            if bv[i] {
                rank1 += 1
            } else {
                rank0 += 1
            }
        }
    }

    #[test]
    fn select<T: FID>() {
        let len = 1000;
        let mut rng = rand::thread_rng();

        let mut bv = vec![false; len];
        for i in 0..len {
            bv[i] = rng.gen();
        }
        let mut fid = T::from_bool_vec(&bv);
        // check if set/unset updates offsets correclty
        for i in 0..len {
            bv[i] = rng.gen();
            fid.set(i, bv[i]);
        }

        let mut prev = 0;
        for i in 0..fid.rank0(fid.len()) {
            let pos = fid.select0(i);
            assert!(!fid.access(pos));
            assert!(i == 0 || pos > prev);
            prev = pos;
        }
        let mut prev = 0;
        for i in 0..fid.rank1(fid.len()) {
            let pos = fid.select1(i);
            assert!(fid.access(pos));
            assert!(i == 0 || pos > prev);
            prev = pos;
        }
    }

    #[test]
    fn not<T: FID + PartialEq + Debug + Not<Output=T>>() {
        let len = 1000;
        let mut rng = rand::thread_rng();
        let mut actual_vec = vec![false;len];
        let mut expected_vec = vec![false;len];

        for i in 0..len {
            actual_vec[i] = rng.gen();
            expected_vec[i] = !actual_vec[i];
        }

        let bv = T::from_bool_vec(&actual_vec);
        let expected = T::from_bool_vec(&expected_vec);
        assert_eq!(expected, !bv);
    }
}
