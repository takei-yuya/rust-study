use super::fid::FID;
use super::fid::NaiveFID;

use crate::collections::heap::Heap;

use std::cmp::Ordering;

pub struct U8WaveletMatrix<T: FID> {
    n: usize,
    matrix: Vec<T>,
    offset: [usize; 256],
}

struct TopKItem {
    s: usize,
    e: usize,
    d: usize,
    v: u8,
}

impl TopKItem {
    fn new(s: usize, e: usize, d: usize, v: u8) -> Self {
        TopKItem{ s, e, d, v }
    }
}

impl <T: FID> U8WaveletMatrix<T> {
    pub fn new(vec: &Vec<u8>) -> Self {
        let n = vec.len();
        let mut matrix = Vec::with_capacity(8);
        let mut vec = vec.clone();
        for i in 0..8 {
            let mut zeros: Vec<u8> = Vec::with_capacity(n);
            let mut ones = Vec::with_capacity(n);

            let mask = !((!0_u8) >> 1) >> i;
            let mut bv = Vec::with_capacity(n);
            for v in vec.iter() {
                if (v & mask) == 0 {
                    bv.push(false);
                    zeros.push(*v);
                } else {
                    bv.push(true);
                    ones.push(*v);
                }
            }
            matrix.push(T::from_bool_vec(&bv));
            vec = zeros;
            vec.append(&mut ones);
        }
        let mut offset = [n; 256];
        for (i, v) in vec.iter().enumerate() {
            if offset[*v as usize] == n {
                offset[*v as usize] = i;
            }
        }
        U8WaveletMatrix {
            n,
            matrix,
            offset,
        }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn access(&self, mut i: usize) -> u8 {
        let mut result = 0;
        for fid in &self.matrix {
            let bit = if fid.access(i) { 1 } else { 0 };
            result = (result << 1) | bit;
            if bit == 0 {
                i = fid.rank0(i);
            } else {
                i = fid.rank0(fid.len()) + fid.rank1(i);
            }
        }
        result
    }

    pub fn rank(&self, v: u8, mut i: usize) -> usize {
        if self.offset[v as usize] == self.n { return 0; }
        if i > self.n {
            i = self.n;
        }
        let mut mask = !(!0_u8 >> 1);
        for fid in &self.matrix {
            i = if (v & mask) == 0 {
                fid.rank0(i)
            } else {
                fid.rank0(fid.len()) + fid.rank1(i)
            };
            mask >>= 1;
        }
        i - self.offset[v as usize]
    }

    pub fn select(&self, v: u8, mut i: usize) -> usize {
        if self.offset[v as usize] == self.n { return self.n; }
        i += self.offset[v as usize];
        let mut mask = 1_u8;
        for fid in self.matrix.iter().rev() {
            i = if (v & mask) == 0 {
                fid.select0(i)
            } else {
                fid.select1(i - fid.rank0(fid.len()))
            };
            mask <<= 1;
        }
        i
    }

    pub fn quantile(&self, mut s: usize, mut e: usize, mut r: usize) -> u8 {
        let mut result = 0;
        for fid in &self.matrix {
            let nzero = fid.rank0(e) - fid.rank0(s);
            if r < nzero {
                result = result << 1;
                s = fid.rank0(s);
                e = fid.rank0(e);
            } else {
                result = result << 1 | 1;
                let zeros = fid.rank0(fid.len());
                s = zeros + fid.rank1(s);
                e = zeros + fid.rank1(e);
                r -= nzero;
            }
        }
        result
    }

    pub fn topk(&self, s: usize, e: usize, k: usize) -> Vec<(u8, usize)> {
        let mut result = vec![];
        let mut heap = Heap::with_compare(|lhs: &TopKItem, rhs|
            // more freq first, small value first
            match ((rhs.e-rhs.s).cmp(&(lhs.e-lhs.s)), lhs.v.cmp(&rhs.v)) {
                (Ordering::Equal, c2) => c2,
                (c1, _) => c1,
            }
        );

        heap.push(TopKItem::new(s, e, 0, 0));
        while let Some(q) = heap.pop() {
            if result.len() >= k {
                break;
            }
            if q.d >= self.matrix.len() {
                result.push((q.v, q.e - q.s));
                continue;
            }
            let fid = &self.matrix[q.d];

            let zs = fid.rank0(q.s);
            let ze = fid.rank0(q.e);
            if zs < ze {
                heap.push(TopKItem::new(zs, ze, q.d + 1, q.v << 1));
            }

            let zeros = fid.rank0(fid.len());
            let os = zeros + fid.rank1(q.s);
            let oe = zeros + fid.rank1(q.e);
            if os < oe {
                heap.push(TopKItem::new(os, oe, q.d + 1, q.v << 1 | 1));
            }
        }
        result
    }
}
pub type NaiveU8WaveletMatrix = U8WaveletMatrix<NaiveFID>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn construct() {
        let u8s = vec![4, 2, 1, 5, 7, 4, 5, 0];
        let wmat = NaiveU8WaveletMatrix::new(&u8s);

        assert_eq!(8, wmat.len());
        assert_eq!(8, wmat.matrix.len());
        assert_eq!(NaiveFID::from_bool_vec(&vec![false, false, false, false, false, false, false, false]), wmat.matrix[0]);
        assert_eq!(NaiveFID::from_bool_vec(&vec![false, false, false, false, false, false, false, false]), wmat.matrix[1]);
        assert_eq!(NaiveFID::from_bool_vec(&vec![false, false, false, false, false, false, false, false]), wmat.matrix[2]);
        assert_eq!(NaiveFID::from_bool_vec(&vec![false, false, false, false, false, false, false, false]), wmat.matrix[3]);
        assert_eq!(NaiveFID::from_bool_vec(&vec![false, false, false, false, false, false, false, false]), wmat.matrix[4]);
        assert_eq!(NaiveFID::from_bool_vec(&vec![true , false, false, true , true , true , true , false]), wmat.matrix[5]);
        assert_eq!(NaiveFID::from_bool_vec(&vec![true , false, false, false, false, true , false, false]), wmat.matrix[6]);
        assert_eq!(NaiveFID::from_bool_vec(&vec![true , false, false, true , false, true , false, true ]), wmat.matrix[7]);

        // B[5]:   4 2 1 5 7 4 5 0      1 0 0 1 1 1 1 0
        // B[6]:   2 1 0 4 5 7 4 5      1 0 0 0 0 1 0 0
        // B[7]:   1 0 4 5 4 5 2 7      1 0 0 1 0 1 0 1
        // offset: 0 4 4 2 1 5 5 7

        let mut expected_offset = [u8s.len(); 256];
        expected_offset[0] = 0;
        expected_offset[4] = 1;
        expected_offset[2] = 3;
        expected_offset[1] = 4;
        expected_offset[5] = 5;
        expected_offset[7] = 7;
        assert_eq!(expected_offset, wmat.offset);
    }

    #[test]
    fn access() {
        let u8s = vec![4, 2, 1, 5, 7, 4, 5, 0];
        let wmat = NaiveU8WaveletMatrix::new(&u8s);

        let mut actual = Vec::with_capacity(u8s.len());
        for i in 0..u8s.len() {
            actual.push(wmat.access(i));
        }
        assert_eq!(u8s, actual);
    }

    #[test]
    fn rank() {
        let u8s = vec![4, 2, 1, 5, 7, 4, 5, 0];
        let wmat = NaiveU8WaveletMatrix::new(&u8s);

        let expected = vec![
            //     [4, 2, 1, 5, 7, 4, 5, 0]
            vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1],  // 0
            vec![0, 0, 0, 1, 1, 1, 1, 1, 1, 1],  // 1
            vec![0, 0, 1, 1, 1, 1, 1, 1, 1, 1],  // 2
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],  // 3
            vec![0, 1, 1, 1, 1, 1, 2, 2, 2, 2],  // 4
            vec![0, 0, 0, 0, 1, 1, 1, 2, 2, 2],  // 5
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],  // 6
            vec![0, 0, 0, 0, 0, 1, 1, 1, 1, 1],  // 7
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],  // 8
        ];
        for n in 0..expected.len() {
            let mut actual = vec![0; expected[n].len()];
            for i in 0..(expected[n].len()) {
                actual[i] = wmat.rank(n as u8, i);
            }
            assert_eq!(expected[n], actual);
        }
    }

    #[test]
    fn rank_same_values() {
        let u8s = vec![3, 3, 3, 3];
        let wmat = NaiveU8WaveletMatrix::new(&u8s);

        let expected = vec![
            vec![0, 0, 0, 0, 0, 0],  // 0
            vec![0, 0, 0, 0, 0, 0],  // 1
            vec![0, 0, 0, 0, 0, 0],  // 2
            vec![0, 1, 2, 3, 4, 4],  // 3
        ];
        for n in 0..expected.len() {
            let mut actual = vec![0; expected[n].len()];
            for i in 0..(expected[n].len()) {
                actual[i] = wmat.rank(n as u8, i);
            }
            assert_eq!(expected[n], actual);
        }
    }

    #[test]
    fn select() {
        let u8s = vec![4, 2, 1, 5, 7, 4, 5, 0];
        let wmat = NaiveU8WaveletMatrix::new(&u8s);

        let expected = vec![
            //  [4, 2, 1, 5, 7, 4, 5, 0]
            vec![                     7, 8],  // 0
            vec![      2,                8],  // 1
            vec![   1,                   8],  // 2
            vec![                        8],  // 3
            vec![0,             5,       8],  // 4
            vec![         3,       6,    8],  // 5
            vec![                        8],  // 6
            vec![            4,          8],  // 7
            vec![                        8],  // 8
        ];
        for n in 0..expected.len() {
            let mut actual = vec![0; expected[n].len()];
            for i in 0..(expected[n].len()) {
                actual[i] = wmat.select(n as u8, i);
            }
            assert_eq!(expected[n], actual);
        }
    }

    #[test]
    fn quantile() {
        let u8s = vec![4, 2, 1, 5, 7, 4, 5, 0];
        let wmat = NaiveU8WaveletMatrix::new(&u8s);

        for s in 0..u8s.len() {
            for e in s..u8s.len() {
                let mut actual = vec![];
                for r in 0..e-s {
                    actual.push(wmat.quantile(s, e, r));
                }
                let mut expected = u8s[s..e].to_vec();
                expected.sort();
                assert_eq!(expected, actual);
            }
        }
    }

    #[test]
    fn topk() {
        let u8s = vec![5, 1, 3, 1, 2, 2, 1, 4];
        let wmat = NaiveU8WaveletMatrix::new(&u8s);

        for s in 0..u8s.len() {
            for e in s..u8s.len() {
                for k in 0..e-s {
                    let mut counts: HashMap<u8, usize> = HashMap::new();
                    for v in &u8s[s..e] {
                        *counts.entry(*v).or_default() += 1;
                    }
                    let mut expected = vec![];
                    for (v, c) in counts {
                        expected.push((v, c));
                    }
                    expected.sort_by(|(v1,c1),(v2,c2)|
                        // more freq first, small value first
                        match (v1.cmp(v2), c2.cmp(c1)) {
                            (c1, Ordering::Equal) => c1,
                            (_, c2) => c2,
                        }
                    );
                    println!("s = {}, e = {}, k = {}", s, e, k);
                    println!("expected = {:?}", expected);
                    if expected.len() > k {
                        expected.resize(k, (0, 0));
                    }

                    let actual = wmat.topk(s, e, k);
                    assert_eq!(expected, actual)
                }
            }
        }
    }

    #[test]
    fn example() {
        let str = "ATCTATGGGAGGAAGAGAAAGTGGAATCTCTGTATCATCTTTCTTAGTCC";
        let u8s = str.as_bytes().to_vec();
        let wmat = NaiveU8WaveletMatrix::new(&u8s);

        //    0         1         2         3         4
        // i: 01234567890123456789012345678901234567890123456789
        //    ATCTATGGGAGGAAGAGAAAGTGGAATCTCTGTATCATCTTTCTTAGTCC
        // A: 0   1    2  34 5 678    90       1  2        3
        // C:   0                        1 2     3  4   5     67
        // G:       012 34  5 6   7 89       0              1
        // T:  0 1 2               3    4 5 6 7 8  9 012 34  5

        // simple count
        assert_eq!(14, wmat.rank('A' as u8, wmat.len()));
        assert_eq!( 8, wmat.rank('C' as u8, wmat.len()));
        assert_eq!(12, wmat.rank('G' as u8, wmat.len()));
        assert_eq!(16, wmat.rank('T' as u8, wmat.len()));

        // count 'T's in [0, 10), [10, 20), [20, 30), [30, 40)
        assert_eq!(3, wmat.rank('T' as u8, 10) - wmat.rank('T' as u8,  0));
        assert_eq!(0, wmat.rank('T' as u8, 20) - wmat.rank('T' as u8, 10));
        assert_eq!(3, wmat.rank('T' as u8, 30) - wmat.rank('T' as u8, 20));
        assert_eq!(5, wmat.rank('T' as u8, 40) - wmat.rank('T' as u8, 30));
        assert_eq!(5, wmat.rank('T' as u8, 50) - wmat.rank('T' as u8, 40));

        // return position 0th, 1st, 2nd, 3th, 4th 'T'
        assert_eq!( 1, wmat.select('T' as u8, 0));
        assert_eq!( 3, wmat.select('T' as u8, 1));
        assert_eq!( 5, wmat.select('T' as u8, 2));
        assert_eq!(21, wmat.select('T' as u8, 3));
        assert_eq!(26, wmat.select('T' as u8, 4));

        // topk
        assert_eq!(
            vec![('T' as u8, 16), ('A' as u8, 14), ('G' as u8, 12), ('C' as u8, 8)],
            wmat.topk(0, u8s.len(), 4)
        );
        assert_eq!(
            vec![('G' as u8, 3), ('T' as u8, 3), ('A' as u8, 2), ('C' as u8, 2)],
            wmat.topk(20, 30, 4)
        );
    }
}
