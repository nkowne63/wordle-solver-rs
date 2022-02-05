// this source code is modified from https://gist.github.com/Leko/125e92a263043debc36f5aa895bfd015
// the reference implementation of leko competiion in rust
use std::io;

#[derive(Debug, Clone)]
pub enum Response {
    Absent,
    Present,
    Correct,
}

#[derive(Debug)]
pub struct History {
    pub word: Vec<char>,
    pub response: Vec<Response>,
}

pub trait Guesser {
    #[allow(clippy::ptr_arg)]
    fn guess(&mut self, history: &Vec<History>) -> String;
}

pub trait LekoRepl: Guesser {
    #[allow(clippy::single_char_pattern)]
    fn run_repl(&mut self) {
        let mut history: Vec<History> = Vec::new();
        let mut word = self.guess(&history);
        println!("{}", word);

        loop {
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            eprintln!("line: {}", line);
            match line.as_str().trim() {
                "NOT_IN_WORD_LIST" => {
                    panic!("You need to give another word")
                }
                _ => {
                    let response: Vec<Response> = line
                        .trim()
                        .split(",")
                        .map(|res| match res {
                            "correct" => Response::Correct,
                            "present" => Response::Present,
                            "absent" => Response::Absent,
                            unknown => panic!("Unrecognized response: {:?}", unknown),
                        })
                        .collect();
                    if response.iter().all(|r| matches!(r, Response::Correct)) {
                        eprintln!("win: {:?}", word);
                        break;
                    }
                    history.push(History {
                        word: word.chars().collect(),
                        response,
                    });
                    eprintln!("{:?}", &history);
                    word = self.guess(&history);
                    println!("{}", word);
                }
            }
        }
    }
}
