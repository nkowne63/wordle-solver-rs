use crate::{
    enums::{Alphabet, Status, StatusChar, Word},
    leko_competition::interface::{Guesser, History, Response},
    tactics::solver::Solver,
};

impl From<Response> for StatusChar {
    fn from(response: Response) -> Self {
        match response {
            Response::Correct => StatusChar::Green,
            Response::Present => StatusChar::Yellow,
            Response::Absent => StatusChar::Gray,
        }
    }
}

impl From<Vec<Response>> for Status {
    fn from(responses: Vec<Response>) -> Self {
        let mut status = Status([StatusChar::Gray; 5]);
        for (i, response) in responses.into_iter().enumerate() {
            status.0[i] = response.into();
        }
        status
    }
}

impl From<Vec<char>> for Word {
    fn from(chars: Vec<char>) -> Self {
        let mut word = [Alphabet::A; 5];
        for (i, c) in chars.into_iter().enumerate() {
            word[i] = c.to_string().parse().unwrap();
        }
        Word(word)
    }
}

impl<S: Solver> Guesser for S {
    fn guess(&mut self, history: &Vec<History>) -> String {
        if history.is_empty() {
            return "soare".to_string();
        }
        for history in history.iter() {
            let status = Status::from(history.response.clone());
            let word = Word::from(history.word.clone());
            self.filter(&word, &status);
        }
        self.next().to_string()
    }
}
