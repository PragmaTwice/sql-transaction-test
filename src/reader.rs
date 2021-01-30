use std::{fs::File, io::{BufReader, Read}};

use bit_vec::BitVec;

use crate::permutation::BinaryPermutation;

/// reading a couple of file and processing with various permutation
struct CoupleReader<R: Read = File> {
    first: BufReader<R>,
    second: BufReader<R>,
    perm: BinaryPermutation,
}

impl <R: Read> CoupleReader<R> {
    /// construct a new CoupleReader with two readable object
    pub fn new(first: R, second: R) -> Self {
        Self {
            first: BufReader::new(first),
            second: BufReader::new(second),
            perm: BinaryPermutation(BitVec::new())
        }
    }
}
