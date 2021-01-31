use bit_vec::{BitBlock, BitVec, Iter};
use std::{cmp::min, ops::Index};

/// merged permutation of two sequence (as `A`, `B`)
///
/// the permutation is stored as a bit vector with length `|A| + |B|`, 
/// and there are `|A|` 0s and `|B|` 1s inside
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BinaryPermutation<B: BitBlock = u32>(pub BitVec<B>);

/// forward Index<usize> for BitVec
impl <B: BitBlock> Index<usize> for BinaryPermutation<B> {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl BinaryPermutation<u32> {
    /// construct an initial permutation of `|A|` 0s followed by `|B|` 1s
    pub fn new(zero_num: usize, one_num: usize) -> Self {
        Self(BitVec::from_fn(zero_num + one_num, |i| i >= zero_num))
    }
}

impl <B: BitBlock> BinaryPermutation<B> {
    /// forward BitVec::len()
    fn len(&self) -> usize {
        self.0.len()
    }

    /// swap value by index
    fn swap(&mut self, lhs: usize, rhs: usize) {
        let tmp = self[lhs];

        self.0.set(lhs, self[rhs]);
        self.0.set(rhs, tmp);
    }

    /// reverse value in [from, to)
    fn reverse(&mut self, mut from: usize, mut to: usize) {
        while from != to && from != { to -= 1; to } {
            self.swap(from, to);
            from += 1;
        }
    }

    /// find the next permutation, and return true if it is found
    ///
    /// otherwise it will be set to the first permutation and return false
    ///
    /// complexity: `O(n)` where `n = lhs + rhs`
    pub fn next(&mut self) -> bool {
        if self.len() < 2 {
            return false;
        }

        let mut less = self.len() - 2;

        while self[less] || !self[less + 1] {
            if less == 0 {
                self.reverse(0, self.len());
                return false;
            }

            less -= 1;
        }

        let mut greater = self.len() - 1;

        while !self[greater] {
            greater -= 1;
        }

        self.swap(less, greater);
        self.reverse(less + 1, self.len());

        true
    }

    /// returns $$C_{lhs + rhs}^{lhs}$$ (imprecise)
    ///
    /// complexity: `O(n)` where `n = lhs + rhs - min(lhs, rhs)`
    pub fn count(lhs: usize, rhs: usize) -> f64 {
        let n = lhs + rhs;
        let m = min(lhs, rhs);

        let mut res = 0f64;

        for i in (m + 1)..(n + 1) {
            res += (i as f64).ln();
        }

        for i in 1..(n - m + 1) {
            res -= (i as f64).ln();
        }

        res.exp()
    }

    /// apply the function `f` in the order of its own permutation to `left` or `right` iterator
    ///
    /// the second parameter of `f` is fed with `true` if `right` is iterated now, `false` otherwise
    pub fn process_iter<T, F>(&self, mut left: T, mut right: T, mut f: F) where
        T : Iterator,
        F : FnMut(T::Item, bool) -> () {
        for i in &self.0 {
            if let Some(x) = if i { &mut right } else { &mut left }.next() {
                f(x, i);
            } else {
                break;
            }
        }
    }

    /// get an iterator of this permutation for two sequence
    pub fn iter<'a, T: Iterator>(&'a self, left: T, right: T) -> BinPermIter<'a, T, B> {
        BinPermIter {
            perm_iter: self.0.iter(),
            left,
            right,
        }
    }
}

/// a merged iterator from two different iterator, iterating with a certain permutation
#[derive(Clone)]
pub struct BinPermIter<'a, T: Iterator, B: 'a + BitBlock = u32> {
    perm_iter: Iter<'a, B>,
    left: T,
    right: T,
}

impl <'a, T: Iterator, B: BitBlock> Iterator for BinPermIter<'a, T, B> {
    type Item = (T::Item, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.perm_iter.next()?;

        if x { &mut self.right } else { &mut self.left }
            .next()
            .map(|y| (y, x))
    }
}

/// convert something to boolean value
trait ToBool {
    fn to_bool(self) -> bool;
}

impl ToBool for bool {
    fn to_bool(self) -> bool {
        self
    }
}

impl ToBool for i32 {
    fn to_bool(self) -> bool {
        self != 0
    }
}

macro_rules! bin_perm {
    [$($x:expr),*] => {{
        let mut bv = BitVec::new();

        $(bv.push($x.to_bool());)*

        BinaryPermutation(bv)
    }}
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_perm_next() {
        let mut bp = BinaryPermutation::new(2, 3);

        assert_eq!(bp, bin_perm![0, 0, 1, 1, 1]);

        assert_eq!(bp.next(), true);
        assert_eq!(bp, bin_perm![0, 1, 0, 1, 1]);

        assert_eq!(bp.next(), true);
        assert_eq!(bp, bin_perm![0, 1, 1, 0, 1]);

        assert_eq!(bp.next(), true);
        assert_eq!(bp, bin_perm![0, 1, 1, 1, 0]);

        assert_eq!(bp.next(), true);
        assert_eq!(bp, bin_perm![1, 0, 0, 1, 1]);

        assert_eq!(bp.next(), true);
        assert_eq!(bp, bin_perm![1, 0, 1, 0, 1]);

        assert_eq!(bp.next(), true);
        assert_eq!(bp, bin_perm![1, 0, 1, 1, 0]);

        assert_eq!(bp.next(), true);
        assert_eq!(bp, bin_perm![1, 1, 0, 0, 1]);

        assert_eq!(bp.next(), true);
        assert_eq!(bp, bin_perm![1, 1, 0, 1, 0]);

        assert_eq!(bp.next(), true);
        assert_eq!(bp, bin_perm![1, 1, 1, 0, 0]);

        assert_eq!(bp.next(), false);
        assert_eq!(bp, bin_perm![0, 0, 1, 1, 1]);
    }

    #[test]
    fn test_process_iter() {
        let mut bp = BinaryPermutation::new(2, 2);

        let l = [1, 2];
        let r = [3, 4];

        let process_iter = |bp: &BinaryPermutation, seq: &[_]| {
            let mut iter = seq.iter(); 
            bp.process_iter(l.iter(), r.iter(), |i, _| {
                assert_eq!(i, iter.next().unwrap());
            })
        };

        process_iter(&bp, &[1, 2, 3, 4]);

        bp.next();
        process_iter(&bp, &[1, 3, 2, 4]);

        bp.next();
        process_iter(&bp, &[1, 3, 4, 2]);

        bp.next();
        process_iter(&bp, &[3, 1, 2, 4]);

        bp.next();
        process_iter(&bp, &[3, 1, 4, 2]);

        bp.next();
        process_iter(&bp, &[3, 4, 1, 2]);
    }

    #[test]
    fn test_iter() {
        let mut bp = BinaryPermutation::new(2, 2);

        let l = [1, 2];
        let r = [3, 4];

        let to_collection = |bp: &BinaryPermutation| bp.iter(l.iter(), r.iter())
            .map(|i| *i.0)
            .collect::<Vec<_>>();

        assert_eq!(to_collection(&bp), vec![1, 2, 3, 4]);

        bp.next();
        assert_eq!(to_collection(&bp), vec![1, 3, 2, 4]);

        bp.next();
        assert_eq!(to_collection(&bp), vec![1, 3, 4, 2]);

        bp.next();
        assert_eq!(to_collection(&bp), vec![3, 1, 2, 4]);

        bp.next();
        assert_eq!(to_collection(&bp), vec![3, 1, 4, 2]);

        bp.next();
        assert_eq!(to_collection(&bp), vec![3, 4, 1, 2]);
    }

    #[test]
    fn test_count() {
        assert_eq!(BinaryPermutation::<u32>::count(0, 0).round() as u128, 1);
        assert_eq!(BinaryPermutation::<u32>::count(1, 0).round() as u128, 1);
        assert_eq!(BinaryPermutation::<u32>::count(0, 1).round() as u128, 1);
        assert_eq!(BinaryPermutation::<u32>::count(1, 1).round() as u128, 2);
        assert_eq!(BinaryPermutation::<u32>::count(2, 2).round() as u128, 6);
        assert_eq!(BinaryPermutation::<u32>::count(2, 3).round() as u128, 10);
        assert_eq!(BinaryPermutation::<u32>::count(3, 3).round() as u128, 20);
        assert_eq!(BinaryPermutation::<u32>::count(5, 5).round() as u128, 252);
    }
}
