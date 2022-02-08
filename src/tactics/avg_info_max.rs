use crate::enums::{Status, Word};
use crate::get_show_console;
use crate::tactics::solver::Solver;
use std::{collections::HashMap, time::Instant};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct WordPair(Word, Word);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct WordStatusPair(Word, Status);

pub struct Board {
    pub remaining_canditates: Vec<Word>,
    input_canditates: Vec<Word>,
}

impl Solver for Board {
    fn new(canditates: Vec<Word>, inputs: Vec<Word>) -> Board {
        Board {
            remaining_canditates: canditates,
            input_canditates: inputs,
        }
    }
    fn filter(&mut self, word: &Word, status: &Status) {
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
        if get_show_console() {
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
    }
    fn next(&self) -> Word {
        if self.remaining_canditates.len() <= 2 {
            return self.remaining_canditates[0];
        }
        let all_start = Instant::now();
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
                if get_show_console() {
                    let percentage = (i_index as f64 / input_len as f64) * 100.0;
                    let percentage = ((percentage / 10.0).floor() as i64 * 10) as f64;
                    if percentage >= (current + 10.0) {
                        current = (current + 10.0).max(percentage);
                        println!("board percentage {}%", current);
                    }
                }
            });
        let end = start.elapsed();
        if get_show_console() {
            println!("board time: {:?}", end);
        }
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
                if get_show_console() {
                    let percentage = (i_index as f64 / input_len as f64) * 100.0;
                    let percentage = ((percentage / 10.0).floor() as i64 * 10) as f64;
                    if percentage >= (current + 10.0) {
                        current = (current + 10.0).max(percentage);
                        println!("info percentage {}%", current);
                    }
                }
            });
        if get_show_console() {
            let end = start.elapsed();
            println!("info time: {:?}", end);
        }
        // search next word
        let start = Instant::now();
        let (word, info) = word_avg_info
            .iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();
        if get_show_console() {
            let end = start.elapsed();
            let all_end = all_start.elapsed();
            println!("search time: {:?}", end);
            println!("next word time: {:?}", all_end);
            println!("next word: {}", word.to_string());
            println!("next word info: {}", info);
        }
        *word
    }
}

#[cfg(test)]
mod tests {
    use crate::set_show_console;

    use super::*;
    #[test]
    fn filter() {
        set_show_console(true);
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
        set_show_console(true);
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
        let next = board.next();
        assert_eq!(next, "afkpz".parse().unwrap());
    }
}

#[cfg(test)]
mod bench {
    use super::*;
    use crate::enums::Word;
    use crate::CANDITATES;
    #[test]
    #[ignore]
    fn get_maximum() {
        let mut board = Board::new(CANDITATES.get_canditates(), CANDITATES.get_all_words());
        board.next();
    }
    #[test]
    #[ignore]
    fn get_avg_count() {
        let best_first = "soare";
        let all_answers = CANDITATES.get_canditates();
        let mut average_count = 0;
        let answer_len = all_answers.len();
        let mut current = 0f64;
        all_answers.iter().enumerate().for_each(|(a_idx, answer)| {
            let mut board = Board::new(CANDITATES.get_canditates(), CANDITATES.get_all_words());
            let first_word: Word = best_first.parse().unwrap();
            let first_status = Word::to_status(&first_word, answer);
            board.filter(&first_word, &first_status);
            average_count += 1;
            loop {
                if board.remaining_canditates.len() == 1 && board.remaining_canditates[0] == *answer
                {
                    average_count += 1;
                    break;
                }
                let next_word = board.next();
                let status = Word::to_status(&next_word, answer);
                board.filter(&next_word, &status);
                average_count += 1;
            }
            let percentage = (a_idx as f64 / answer_len as f64) * 100.0;
            let percentage = ((percentage / 5.0).floor() as i64 * 5) as f64;
            if percentage >= (current + 5.0) {
                current = (current + 5.0).max(percentage);
                println!("board percentage {}%", current);
            }
        });
        let average_count = average_count as f64 / all_answers.len() as f64;
        println!("average count: {}", average_count);
    }
}
