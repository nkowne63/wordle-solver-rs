use crate::enums::{Status, Word};
use std::{collections::HashMap, time::Instant};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct WordPair(Word, Word);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct WordStatusPair(Word, Status);

pub struct Board {
    pub remaining_canditates: Vec<Word>,
    input_canditates: Vec<Word>,
}

impl Board {
    pub fn new(canditates: Vec<Word>, inputs: Vec<Word>) -> Board {
        Board {
            remaining_canditates: canditates,
            input_canditates: inputs,
        }
    }
    pub fn filter(&mut self, word: &Word, status: &Status) {
        let start = Instant::now();
        let &mut Board {
            ref mut remaining_canditates,
            input_canditates: _,
        } = self;
        let before_len = remaining_canditates.len();
        let remaining: Vec<Word> = remaining_canditates
            .iter()
            .filter(|&answer| status == &Word::to_status(word, answer))
            .cloned()
            .collect();
        let after_len = remaining.len();
        *remaining_canditates = remaining;
        let end = start.elapsed();
        if remaining_canditates.len() > 2 {
            println!(
                "first three: {}, {}, {}",
                remaining_canditates[0].to_string(),
                remaining_canditates[1].to_string(),
                remaining_canditates[2].to_string()
            );
        } else {
            println!(
                "remaining: {:?}",
                remaining_canditates
                    .iter()
                    .map(|w| w.to_string())
                    .collect::<Vec<String>>()
            );
        }
        println!("filter: {} -> {}", before_len, after_len);
        println!(
            "gained information: {}",
            (before_len as f64 / after_len as f64).log2()
        );
        println!("filter time: {:?}", end);
    }
    pub fn compute_next_word_info(&self) -> (Word, f64) {
        let start = Instant::now();
        let &Board {
            ref remaining_canditates,
            ref input_canditates,
        } = self;
        // construct board
        let mut status_board = HashMap::new();
        let input_len = input_canditates.len() as u32;
        let mut current = 0f64;
        input_canditates
            .iter()
            .enumerate()
            .for_each(|(i_index, word)| {
                remaining_canditates
                    .iter()
                    .enumerate()
                    .for_each(|(_a_index, answer)| {
                        let status = Word::to_status(word, answer);
                        status_board.insert(WordPair(*word, *answer), status);
                    });
                let percentage = (i_index as f64 / input_len as f64) * 100.0;
                let percentage = ((percentage / 10.0).floor() as i64 * 10) as f64;
                if percentage >= (current + 10.0) {
                    current = (current + 10.0).max(percentage);
                    println!("board percentage {}%", current);
                }
            });
        let end = start.elapsed();
        println!("board time: {:?}", end);
        // construct color groping and word_avg_info
        let start = Instant::now();
        let len_of_remaining_canditates = remaining_canditates.len() as u32;
        let input_len = input_canditates.len() as u32;
        let mut current = 0f64;
        let mut word_color_grouping = HashMap::new();
        let mut word_avg_info = HashMap::new();
        input_canditates
            .iter()
            .enumerate()
            .for_each(|(i_index, word)| {
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
                    let probablity = count as f64 / len_of_remaining_canditates as f64;
                    avg_info += -probablity * probablity.log2();
                });
                word_avg_info.insert(*word, avg_info);
                let percentage = (i_index as f64 / input_len as f64) * 100.0;
                let percentage = ((percentage / 10.0).floor() as i64 * 10) as f64;
                if percentage >= (current + 10.0) {
                    current = (current + 10.0).max(percentage);
                    println!("info percentage {}%", current);
                }
            });
        let end = start.elapsed();
        println!("info time: {:?}", end);
        // search next word
        let start = Instant::now();
        let (word, info) = word_avg_info
            .iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();
        let end = start.elapsed();
        println!("search time: {:?}", end);
        println!("next word: {}", word.to_string());
        println!("next word info: {}", info);
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
        let board = Board::new(
            vec!["abcde", "fghij", "klmno", "pqrst"]
                .iter()
                .map(|s| s.parse().unwrap())
                .collect(),
            vec!["afkpz", "fghij", "klmno", "pqrst"]
                .iter()
                .map(|s| s.parse().unwrap())
                .collect(),
        );
        let (next, info) = board.compute_next_word_info();
        assert_eq!(next, "afkpz".parse().unwrap());
        assert_eq!(info, 2f64);
    }
}
