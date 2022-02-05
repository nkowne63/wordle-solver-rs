use crate::enums::{Status, Word};

pub trait Solver {
    fn new(canditates: Vec<Word>, inputs: Vec<Word>) -> Self;
    fn filter(&mut self, word: &Word, status: &Status);
    fn next(&self) -> Word;
}
