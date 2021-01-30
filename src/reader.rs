use std::{fs::File, io::{BufRead, BufReader, Read, Result, Seek, SeekFrom, Lines}};

use crate::permutation::{BinPermIter, BinaryPermutation};

#[derive(Debug)]
/// reading a couple of file and processing with various permutation
struct CoupleReader<R = File> {
    first: R,
    second: R,
    perm: BinaryPermutation,
}

trait TryClone {
    fn try_clone(&self) -> Result<Self> where Self: Sized;
}

impl TryClone for File {
    fn try_clone(&self) -> Result<Self> {
        self.try_clone()
    }
}

impl <R: Read + TryClone + Seek> CoupleReader<R> {
    /// construct a new CoupleReader with two readable object
    pub fn new(mut first: R, mut second: R) -> Result<Self> {
        let first_len = Self::get_lines(&mut first)?.count();
        let second_len = Self::get_lines(&mut second)?.count();

        Ok(Self {
            first,
            second,
            perm: BinaryPermutation::new(first_len, second_len),
        })
    }

    /// retrieve number of lines
    fn get_lines(x: &mut R) -> Result<Lines<BufReader<R>>> {
        let lines = BufReader::new(x.try_clone()?).lines();

        x.seek(SeekFrom::Start(0))?;

        Ok(lines)
    }

    pub fn next(&mut self) -> bool {
        self.perm.next()
    }

    pub fn iter<'a>(&'a mut self) -> Result<BinPermIter<'a, Lines<BufReader<R>>>> {
        let left = Self::get_lines(&mut self.first)?;
        let right = Self::get_lines(&mut self.second)?;

        Ok(self.perm.iter(left, right))
    }
}
