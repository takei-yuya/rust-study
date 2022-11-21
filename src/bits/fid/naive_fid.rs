use super::FID;

#[derive(Clone, Debug)]
pub struct NaiveFID {
    n: usize,
    blocks: Vec<u64>,
    popcount_offset: Vec<usize>,
}

impl NaiveFID {
    fn construct_popcount_offset(blocks: &Vec<u64>) -> Vec<usize> {
        let mut popcount_offset = Vec::with_capacity(blocks.len());
        let mut popcount = 0;
        for block in blocks {
            popcount_offset.push(popcount);
            popcount += block.count_ones() as usize;
        }
        popcount_offset
    }
}

impl FID for NaiveFID {
    fn new(n: usize) -> Self {
        let block_count = n / 64 + 1;
        let mut blocks = Vec::with_capacity(block_count);
        blocks.resize(block_count, 0u64);

        let mut popcount_offset = Vec::with_capacity(block_count);
        popcount_offset.resize(block_count, 0);

        NaiveFID {
            n,
            blocks,
            popcount_offset,
        }
    }

    fn from_bool_vec(vec: &Vec<bool>) -> Self {
        let n = vec.len();
        let block_count = n / 64 + 1;

        let mut blocks: Vec<u64> = Vec::with_capacity(block_count);
        blocks.resize(block_count, 0u64);
        for (i, b) in vec.iter().enumerate() {
            let block = i / 64;
            let index = i % 64;
            if *b {
                blocks[block] |= 1 << index;
            }
        }

        let popcount_offset = Self::construct_popcount_offset(&blocks);

        NaiveFID {
            n,
            blocks,
            popcount_offset,
        }
    }

    fn get(&self, i: usize) -> bool {
        assert!(i < self.n);
        let block_idx = i / 64;
        let bit_idx = i - block_idx * 64;
        let mask = 1u64 << bit_idx;
        (self.blocks[block_idx] & mask) != 0
    }

    fn set(&mut self, i: usize, bit: bool) -> () {
        assert!(i < self.n);
        let block_idx = i / 64;
        let bit_idx = i - block_idx * 64;
        let mask = 1u64 << bit_idx;
        let cur_bit = (self.blocks[block_idx] & mask) != 0;
        if cur_bit == bit {
            return;
        }

        if bit {
            self.blocks[block_idx] |= mask;
            for i in block_idx + 1 .. self.popcount_offset.len() {
                self.popcount_offset[i] += 1;
            }
        } else {
            self.blocks[block_idx] &= !mask;
            for i in block_idx + 1 .. self.popcount_offset.len() {
                self.popcount_offset[i] -= 1;
            }
        }
    }

    fn len(&self) -> usize { self.n }
    fn access(&self, i: usize) -> bool { self.get(i) }
    fn rank1(&self, i: usize) -> usize {
        assert!(i <= self.n);
        let block_idx = i / 64;
        let bit_idx = i - block_idx * 64;
        let mask = if bit_idx == 0 { 0 } else { (!0_u64) >> (64 - bit_idx) };
        self.popcount_offset[block_idx] + (self.blocks[block_idx] & mask).count_ones() as usize
    }
}

impl std::ops::Not for NaiveFID {
    type Output = Self;
    fn not(self) -> Self::Output {
        let mut n = self.n;

        let mut blocks = Vec::with_capacity(self.blocks.len());
        for b in self.blocks {
            if n >= 64 {
                blocks.push(!b);
                n -= 64;
            } else {
                let nb = !b & (!0_u64 >> (64 - n));
                blocks.push(nb);
            }
        }

        let popcount_offset = Self::construct_popcount_offset(&blocks);

        NaiveFID {
            n: self.n,
            blocks,
            popcount_offset,
        }
    }
}

impl PartialEq for NaiveFID {
    fn eq(&self, other: &Self) -> bool {
        if self.n != other.n {
            return false;
        }
        self.blocks == other.blocks
    }
}
