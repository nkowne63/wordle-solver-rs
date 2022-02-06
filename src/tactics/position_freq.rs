use itertools::{iproduct, Itertools};

use crate::enums::{Alphabet, Status, Word};
use crate::get_show_console;
use crate::tactics::solver::Solver;
use std::{collections::HashMap, time::Instant};

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
        if self.remaining_canditates.len() == 1 {
            return self.remaining_canditates[0];
        }
        let start = Instant::now();
        let &Board {
            ref remaining_canditates,
            ref input_canditates,
        } = self;

        // construct char_position_count map
        let mut char_position_count: HashMap<(Alphabet, usize), i32> = HashMap::new();
        iproduct!(remaining_canditates.iter(), 0..5).for_each(|(word, index)| {
            let target_char = word.0[index];
            if let Some(value) = char_position_count.get_mut(&(target_char, index)) {
                *value += 1;
            } else {
                char_position_count.insert((target_char, index), 1);
            }
        });

        // search and calculate max info
        let mut current_max_info = -1f64;
        let mut current_max_word: Word = "qqqqq".parse().unwrap();
        input_canditates.iter().for_each(|word| {
            let mut info = 0f64;
            word.0.iter().enumerate().for_each(|(index, alphabet)| {
                // 各文字ごとに情報量を計算する
                // greenの確率
                let probablity_green = *char_position_count.get(&(*alphabet, index)).unwrap_or(&0)
                    as f64
                    / remaining_canditates.len() as f64;
                // grayの確率
                let probablity_gray = (0..5)
                    .map(|index| {
                        (remaining_canditates.len() as i32
                            - *char_position_count.get(&(*alphabet, index)).unwrap_or(&0))
                            as f64
                    })
                    .reduce(|prev, current| prev * current / remaining_canditates.len() as f64)
                    .unwrap_or(0.0)
                    / remaining_canditates.len() as f64;
                // yellowの確率
                let probablity_yellow = 1f64 - probablity_green - probablity_gray;
                // 情報量の加算
                // しかし、同じ文字がすでに加算されていたら加算しない（ペナルティー）
                if word.0.iter().find_position(|a| a == &alphabet).unwrap().0 != index {
                    return;
                }
                info += vec![probablity_green, probablity_gray, probablity_yellow]
                    .iter()
                    .filter(|&&p| p != 0.0)
                    .map(|probablity| -probablity * probablity.log2())
                    .sum::<f64>();
            });
            // infoが最大だったら更新
            if info > current_max_info {
                current_max_info = info;
                current_max_word = *word;
            }
        });

        let end = start.elapsed();
        if get_show_console() {
            println!("quasi info: {:?}", current_max_info);
            println!("next: {:?}", current_max_word.to_string());
            println!("next time: {:?}", end);
        }

        current_max_word
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::Word;
    use crate::tactics::solver::Solver;
    use crate::CANDITATES;
    #[test]
    #[ignore]
    fn freq_get_maximum() {
        let board = Board::new(CANDITATES.get_canditates(), CANDITATES.get_all_words());
        let word = board.next();
        println!("best first: {}", word.to_string());
    }
    #[test]
    #[ignore]
    fn freq_get_avg_count() {
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
