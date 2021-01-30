use bit_vec::BitVec;
use std::ops::Index;

/// merged permutation of two sequence (as `A`, `B`)
///
/// the permutation is stored as a bit vector with length `|A| + |B|`, 
/// and there are `|A|` 0s and `|B|` 1s inside
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BinaryPermutation(BitVec);

/// forward Index<usize> for BitVec
impl Index<usize> for BinaryPermutation {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl BinaryPermutation {
    /// construct an initial permutation of `|A|` 0s followed by `|B|` 1s
    pub fn new(zero_num: usize, one_num: usize) -> Self {
        Self(BitVec::from_fn(zero_num + one_num, |i| i >= zero_num))
    }

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
    /// complexity: O(n)
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

    /// apply the function `f` in the order of its own permutation to `left` or `right` iterator
    pub fn process_iter<T, F>(&self, mut left: T, mut right: T, f: F) where
        T : Iterator,
        F : Fn(T::Item) -> () {
        for i in &self.0 {
            if let Some(x) = if i { &mut right } else { &mut left }.next() {
                f(x);
            }
        }
    }
}
