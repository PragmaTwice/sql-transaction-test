use std::{fs::File, io::{BufRead, BufReader, Cursor, Lines, Read, Result, Seek, SeekFrom, Error}};

use crate::permutation::{BinPermIter, BinaryPermutation};

#[derive(Debug)]
/// reading a couple of file and processing with various permutation
pub struct CoupleReader<R = File> {
    /// the first IO object
    first: R,

    /// line number of first object
    first_len: usize,

    /// the second IO object
    second: R,

    /// line number of second object
    second_len: usize,

    /// permutation transformer
    perm: BinaryPermutation,
}

pub trait TryClone {
    fn try_clone(&self) -> Result<Self> where Self: Sized;
}

impl TryClone for File {
    fn try_clone(&self) -> Result<Self> {
        self.try_clone()
    }
}

impl <T: Clone> TryClone for Cursor<T> {
    fn try_clone(&self) -> Result<Self> {
        Ok(self.clone())
    }
}

impl <R: Read + TryClone + Seek> CoupleReader<R> {
    /// construct a new CoupleReader with two readable objects
    pub fn new(mut first: R, mut second: R) -> Result<Self> {
        let first_len = Self::get_lines(&mut first)?.count();
        let second_len = Self::get_lines(&mut second)?.count();

        Ok(Self {
            first,
            first_len,
            second,
            second_len,
            perm: BinaryPermutation::new(first_len, second_len),
        })
    }

    /// retrieve number of lines
    fn get_lines(x: &mut R) -> Result<Lines<BufReader<R>>> {
        let lines = BufReader::new(x.try_clone()?).lines();

        x.seek(SeekFrom::Start(0))?;

        Ok(lines)
    }

    /// forward BinaryPermutation::next
    pub fn next(&mut self) -> bool {
        self.perm.next()
    }

    /// forward BinaryPermutation::count
    pub fn count(&self) -> f64 {
        BinaryPermutation::<u32>::count(self.first_len, self.second_len)
    }

    /// get an iterator of this permutation for two readable objects
    pub fn iter<'a>(&'a mut self) -> Result<BinPermIter<'a, Lines<BufReader<R>>>> {
        let left = Self::get_lines(&mut self.first)?;
        let right = Self::get_lines(&mut self.second)?;

        Ok(self.perm.iter(left, right))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reader() -> Result<()> {
        let a = Cursor::new(String::from("hello\nhi"));
        let b = Cursor::new(String::from("once\r\ntwice\n"));

        let mut reader = CoupleReader::new(a, b)?;

        let to_collection = |r: &mut CoupleReader<_>| Ok::<_, Error>(
            r.iter()?.map(|x| x.0.unwrap()).collect::<Vec<_>>()
        );

        assert_eq!(vec!["hello", "hi", "once", "twice"], to_collection(&mut reader)?);

        reader.next();
        assert_eq!(vec!["hello", "once", "hi", "twice"], to_collection(&mut reader)?);

        reader.next();
        assert_eq!(vec!["hello", "once", "twice", "hi"], to_collection(&mut reader)?);

        reader.next();
        assert_eq!(vec!["once", "hello", "hi", "twice"], to_collection(&mut reader)?);

        reader.next();
        assert_eq!(vec!["once", "hello", "twice", "hi"], to_collection(&mut reader)?);

        reader.next();
        assert_eq!(vec!["once", "twice", "hello", "hi"], to_collection(&mut reader)?);

        Ok(())
    }
}
