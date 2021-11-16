use std::cell::{RefCell, RefMut, Cell};
use std::slice::Iter;
use std::borrow::Borrow;

#[derive(Debug, Clone, PartialEq)]
pub enum RandGen {
    Seq(SeqRand),
    Live,
}

impl RandGen {
    pub fn next(&self) -> f64 {
        match self {
            RandGen::Seq(seq_rand) => {
                // return the next item. Crash the program if
                // iterator still can't return anything
                seq_rand.maybe_next().expect("Can't fetch next value")
            },
            RandGen::Live => rand::random()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SeqRand {
    seq: Vec<f64>,
    current_index: Cell<usize>,
}

impl SeqRand {
    pub fn new(seq: Vec<f64>) -> SeqRand {
        SeqRand {
            seq,
            current_index: Cell::new(0),
        }
    }

    pub fn maybe_next(&self) -> Option<f64> {
        if self.seq.is_empty() {
            None
        } else {
            let current_index = self.current_index.get();
            let result = Some(self.seq[current_index % self.seq.len()]);
            self.current_index.set(current_index + 1);
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::random::{RandGen, SeqRand};

    /// Checks that a random number generator returns
    /// a cyclic sequence of numbers
    #[test]
    fn test_random_cyclic_sequence() {
        let gen = RandGen::Seq(SeqRand::new(vec![0.1, 0.5, 1.0]));
        assert_eq!(gen.next(), 0.1);
        assert_eq!(gen.next(), 0.5);
        assert_eq!(gen.next(), 1.0);
        assert_eq!(gen.next(), 0.1);
    }
}
