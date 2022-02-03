use crate::enums::{Status, Word};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct WordPair(Word, Word);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct WordStatusPair(Word, Status);

pub(crate) struct Board {
    status_board: HashMap<WordPair, Status>,
    remaining_canditates: Vec<Word>,
    input_canditates: Vec<Word>,
    word_color_grouping: HashMap<WordStatusPair, u32>,
    word_avg_info: HashMap<Word, f64>,
}

impl Board {
    pub(crate) fn new(canditates: Vec<Word>, inputs: Vec<Word>) -> Board {
        let status_board = HashMap::new();
        let word_color_grouping = HashMap::new();
        let word_avg_info = HashMap::new();
        Board {
            status_board,
            remaining_canditates: canditates,
            input_canditates: inputs,
            word_color_grouping,
            word_avg_info,
        }
    }
    fn filter(&mut self, word: &Word, status: &Status) {
        let &mut Board {
            status_board: _,
            ref mut remaining_canditates,
            input_canditates: _,
            word_color_grouping: _,
            word_avg_info: _,
        } = self;
        let remaining = remaining_canditates
            .iter()
            .filter(|&answer| status == &Word::to_status(word, answer))
            .cloned()
            .collect();
        *remaining_canditates = remaining;
    }
    fn compute_board(&mut self) {
        let &mut Board {
            ref mut remaining_canditates,
            ref mut input_canditates,
            ref mut status_board,
            word_color_grouping: _,
            word_avg_info: _,
        } = self;
        *status_board = HashMap::new();
        input_canditates.iter().for_each(|word| {
            remaining_canditates.iter().for_each(|answer| {
                let status = Word::to_status(word, answer);
                status_board.insert(WordPair(*word, *answer), status);
            });
        });
    }
    fn compute_info(&mut self) {
        let &mut Board {
            ref mut remaining_canditates,
            ref mut input_canditates,
            ref mut status_board,
            ref mut word_color_grouping,
            ref mut word_avg_info,
        } = self;
        let len_of_input_canditates = input_canditates.len() as u32;
        input_canditates.iter().for_each(|word| {
            remaining_canditates.iter().for_each(|answer| {
                let status = status_board.get(&WordPair(*word, *answer)).unwrap();
                word_color_grouping.insert(
                    WordStatusPair(*word, *status),
                    *word_color_grouping
                        .get(&WordStatusPair(*word, *status))
                        .unwrap_or(&0)
                        + 1,
                );
            });
            let mut avg_info = 0f64;
            Status::get_status_iter().for_each(|status| {
                let count = *word_color_grouping
                    .get(&WordStatusPair(*word, status))
                    .unwrap_or(&0);
                if count == 0 {
                    return;
                }
                let probablity = count as f64 / len_of_input_canditates as f64;
                avg_info += -probablity * probablity.log2();
            });
            word_avg_info.insert(*word, avg_info);
        });
    }
    fn compute_next_word_info(&self) -> (Word, f64) {
        let &Board {
            remaining_canditates: _,
            input_canditates: _,
            status_board: _,
            word_color_grouping: _,
            ref word_avg_info,
        } = self;
        let (word, info) = word_avg_info
            .iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();
        (*word, *info)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn filter() {
        let mut board = Board::new(
            vec!["abcde", "fghij", "klmno", "pqrst"]
                .iter()
                .map(|s| s.parse().unwrap())
                .collect(),
            vec!["afkpz", "fghij", "klmno", "pqrst"]
                .iter()
                .map(|s| s.parse().unwrap())
                .collect(),
        );
        board.filter(&"afpkz".parse().unwrap(), &"_y___".parse().unwrap());
        assert_eq!(board.remaining_canditates, vec!["fghij".parse().unwrap()]);
    }
    #[test]
    fn info() {
        let mut board = Board::new(
            vec!["abcde", "fghij", "klmno", "pqrst"]
                .iter()
                .map(|s| s.parse().unwrap())
                .collect(),
            vec!["afkpz", "fghij", "klmno", "pqrst"]
                .iter()
                .map(|s| s.parse().unwrap())
                .collect(),
        );
        board.compute_board();
        board.compute_info();
        let (next, info) = board.compute_next_word_info();
        assert_eq!(next, "afkpz".parse().unwrap());
        assert_eq!(info, 2f64);
    }
}
