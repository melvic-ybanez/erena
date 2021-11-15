use std::cell::{RefCell, RefMut};
use std::slice::Iter;

pub trait Random {
    fn next(&mut self) -> f64;
}

pub struct TestRand {
    seq: Vec<f64>,
    current_index: usize,
}

impl TestRand {
    pub fn from_seq(seq: Vec<f64>) -> TestRand {
        TestRand {
            seq,
            current_index: 0,
        }
    }

    pub fn maybe_next(&mut self) -> Option<f64> {
        if self.seq.is_empty() {
            None
        } else {
            let result = Some(self.seq[self.current_index % self.seq.len()]);
            self.current_index += 1;
            result
        }
    }
}

impl Random for f64 {
    fn next(&mut self) -> f64 {
        rand::random()
    }
}

impl Random for TestRand {
    fn next(&mut self) -> f64 {
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
        let mut gen = TestRand::from_seq(vec![0.1, 0.5, 1.0]);
        assert_eq!(gen.next(), 0.1);
        assert_eq!(gen.next(), 0.5);
        assert_eq!(gen.next(), 1.0);
        assert_eq!(gen.next(), 0.1);
    }
}
