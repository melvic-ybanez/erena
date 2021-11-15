use std::cell::{RefCell, RefMut};
use std::slice::Iter;

pub trait Random<'a> {
    fn next(&'a self) -> f64;
}

pub struct TestRand<'a> {
    seq: Vec<f64>,
    iter: RefCell<Option<Iter<'a, f64>>>,
}

impl<'a> TestRand<'a> {
    pub fn from_seq(seq: Vec<f64>) -> TestRand<'a> {
        TestRand {
            seq,
            iter: RefCell::new(None),
        }
    }

    pub fn maybe_next(&'a self) -> Option<f64> {
        let mut maybe_iter = self.iter.borrow_mut();
        let mut reset = |mut maybe_iter: RefMut<Option<Iter<'a, f64>>>| {
            let mut new_iter = self.seq.iter();
            let result = new_iter.next().cloned();

            // reset iterator to make the set of options cyclic
            *maybe_iter = Some(new_iter);

            // return the next item again
            result
        };

        match maybe_iter.clone() {
            None => reset(maybe_iter),
            Some(mut iter) => iter.next().cloned().or_else(|| reset(maybe_iter)),
        }
    }
}

impl<'a> Random<'a> for f64 {
    fn next(&'a self) -> f64 {
        rand::random()
    }
}

impl<'a> Random<'a> for TestRand<'a> {
    fn next(&'a self) -> f64 {
        // return the next item. Crash the program if
        // iterator still can't return anything
        self.maybe_next().expect("Can't fetch next value")
    }
}

#[cfg(test)]
mod tests {
    use crate::math::random::{Random, TestRand};

    /// Checks that a random number generator returns
    /// a cyclic sequence of numbers
    #[test]
    fn test_random_cyclic_sequence() {
        let gen = TestRand::from_seq(vec![0.1, 0.5, 1.0]);
        assert_eq!(gen.next(), 0.1);
        assert_eq!(gen.next(), 0.5);
        assert_eq!(gen.next(), 1.0);
        assert_eq!(gen.next(), 0.1);
    }
}
