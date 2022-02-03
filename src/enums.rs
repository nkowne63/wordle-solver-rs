use itertools::{iproduct, Itertools};
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
enum Alphabet {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    // statusを計算するときだけ現れるやつ
    Omega,
}

impl ToString for Alphabet {
    fn to_string(&self) -> String {
        match self {
            Alphabet::A => "A",
            Alphabet::B => "B",
            Alphabet::C => "C",
            Alphabet::D => "D",
            Alphabet::E => "E",
            Alphabet::F => "F",
            Alphabet::G => "G",
            Alphabet::H => "H",
            Alphabet::I => "I",
            Alphabet::J => "J",
            Alphabet::K => "K",
            Alphabet::L => "L",
            Alphabet::M => "M",
            Alphabet::N => "N",
            Alphabet::O => "O",
            Alphabet::P => "P",
            Alphabet::Q => "Q",
            Alphabet::R => "R",
            Alphabet::S => "S",
            Alphabet::T => "T",
            Alphabet::U => "U",
            Alphabet::V => "V",
            Alphabet::W => "W",
            Alphabet::X => "X",
            Alphabet::Y => "Y",
            Alphabet::Z => "Z",
            Alphabet::Omega => "_",
        }
        .to_string()
        .to_lowercase()
    }
}

impl FromStr for Alphabet {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s.to_uppercase().as_ref() {
            "A" => Ok(Alphabet::A),
            "B" => Ok(Alphabet::B),
            "C" => Ok(Alphabet::C),
            "D" => Ok(Alphabet::D),
            "E" => Ok(Alphabet::E),
            "F" => Ok(Alphabet::F),
            "G" => Ok(Alphabet::G),
            "H" => Ok(Alphabet::H),
            "I" => Ok(Alphabet::I),
            "J" => Ok(Alphabet::J),
            "K" => Ok(Alphabet::K),
            "L" => Ok(Alphabet::L),
            "M" => Ok(Alphabet::M),
            "N" => Ok(Alphabet::N),
            "O" => Ok(Alphabet::O),
            "P" => Ok(Alphabet::P),
            "Q" => Ok(Alphabet::Q),
            "R" => Ok(Alphabet::R),
            "S" => Ok(Alphabet::S),
            "T" => Ok(Alphabet::T),
            "U" => Ok(Alphabet::U),
            "V" => Ok(Alphabet::V),
            "W" => Ok(Alphabet::W),
            "X" => Ok(Alphabet::X),
            "Y" => Ok(Alphabet::Y),
            "Z" => Ok(Alphabet::Z),
            _ => Err("not a char".to_string()),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
enum StatusChar {
    Gray,
    Yellow,
    Green,
}

impl FromStr for StatusChar {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "g" => Ok(StatusChar::Green),
            "y" => Ok(StatusChar::Yellow),
            "_" => Ok(StatusChar::Gray),
            _ => Err("not a valid status".to_string()),
        }
    }
}

impl ToString for StatusChar {
    fn to_string(&self) -> String {
        match self {
            StatusChar::Gray => "_",
            StatusChar::Yellow => "y",
            StatusChar::Green => "g",
        }
        .to_string()
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct Word([Alphabet; 5]);

impl FromStr for Word {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        let chars = s.chars();
        let mut word = [Alphabet::A; 5];
        chars.into_iter().enumerate().for_each(|(i, c)| {
            word[i] = c.to_string().parse().unwrap();
        });
        Ok(Word(word))
    }
}

impl ToString for Word {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct Status([StatusChar; 5]);

impl FromStr for Status {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        let chars = s.chars();
        let mut status = [StatusChar::Gray; 5];
        chars.into_iter().enumerate().for_each(|(i, c)| {
            status[i] = c.to_string().parse().unwrap();
        });
        Ok(Status(status))
    }
}

impl ToString for Status {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("")
    }
}

impl Status {
    pub fn get_status_iter() -> impl Iterator<Item = Status> {
        iproduct!(
            [StatusChar::Gray, StatusChar::Yellow, StatusChar::Green],
            [StatusChar::Gray, StatusChar::Yellow, StatusChar::Green],
            [StatusChar::Gray, StatusChar::Yellow, StatusChar::Green],
            [StatusChar::Gray, StatusChar::Yellow, StatusChar::Green],
            [StatusChar::Gray, StatusChar::Yellow, StatusChar::Green]
        )
        .map(|(a, b, c, d, e)| Status([a, b, c, d, e]))
    }
}

impl Word {
    // 考えるのが面倒なのでこれのpythonのコードで
    // https://xcloche.hateblo.jp/entry/2022/01/24/212558
    pub fn to_status(word: &Word, answer: &Word) -> Status {
        let mut status = Status([StatusChar::Gray; 5]);
        let mut answer = *answer;
        for word_idx in 0..5 {
            if word.0[word_idx] == answer.0[word_idx] {
                status.0[word_idx] = StatusChar::Green;
                answer.0[word_idx] = Alphabet::Omega;
            }
        }
        for word_idx in 0..5 {
            if answer.0.contains(&word.0[word_idx]) && status.0[word_idx] == StatusChar::Gray {
                status.0[word_idx] = StatusChar::Yellow;
                let pos = answer
                    .0
                    .iter()
                    .find_position(|a| *a == &word.0[word_idx])
                    .unwrap()
                    .0;
                answer.0[pos] = Alphabet::Omega;
            }
        }
        status
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn word() {
        let word: Word = "acegi".parse().unwrap();
        let answer: Word = "abcde".parse().unwrap();
        let status = Word::to_status(&word, &answer);
        assert_eq!(status.to_string(), "gyy__".to_string());
    }
    #[test]
    fn duplicated() {
        let word: Word = "alpha".parse().unwrap();
        let answer: Word = "abcde".parse().unwrap();
        let status = Word::to_status(&word, &answer);
        assert_eq!(status.to_string(), "g____".to_string());
    }
}
