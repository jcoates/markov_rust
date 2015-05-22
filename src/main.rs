//main.rs - commandline interface for doing markov stuff

extern crate readline;

mod markov;

use std::ffi::CString;
use std::fs::File;
use std::io::BufReader;
use std::path::{PathBuf, Path};
use std::str::from_utf8;

use markov::MarkovChain;

#[allow(unused)]
fn main() {
    let mut markov : Option<MarkovChain> = None;
    
    let prompt = CString::new("-> ").unwrap();
    while let Ok(line) = readline::readline(&prompt) {  
        let line = match from_utf8(&*line.to_bytes()) {
            Ok(l) => l,
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        };
        match parse_line(line) {
            Command::Blank => (),
            Command::Exit => break,
            Command::Invalid(msg) => {
                println!("{}", msg);
            },
            Command::Create(degree) => {
                markov = Some(MarkovChain::new(degree));
                println!("Created {}-gram Markov Chain", degree);
            },
            Command::Train(path) => {
                match markov {
                    Some(ref mut m) => {
                        train_chain(m, path);
                    },
                    None => {
                        println!("Please create markov chain first");
                    },
                }
            },
            Command::Generate(n) => {
                match markov {
                    Some(ref mut m) => {
                        let mut x = 0;
                        while x < n  {
                            println!("{}", join_sentence(m.create_sentence()));
                            x += 1;
                        }
                    },
                    None => {
                        println!("Please create markov chain first");
                    },
                }
            }
        }
    }
}

// train_chain: given a markov chain and a path buffer, trains
// the markov chain on the file at the given path
fn train_chain(m : &mut MarkovChain, path : PathBuf) {
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => {
            println!("Could not open file {}", path.to_str().unwrap());
            return;
        }
    };
    let read_file = &mut BufReader::new(file);
    println!("Training on data found in {}", path.to_str().unwrap());
    m.add_training_data(read_file);
    println!("Training complete");
}

// join_sentence : cruddily joins an array of strings with spaces
fn join_sentence(v : Vec<String>) -> String {
    let mut s = String::new();
    let mut i = 0;
    while i < v.len() {
        s.push_str(&v[i]);
        s.push(' ');
        i += 1;
    }
    s
}

// parse_line: given a line from cli, parses it into a command
fn parse_line(s : &str) -> Command {
    let word_vec : Vec<&str> = s.trim().split(' ').collect();
    if word_vec.is_empty() {
        return Command::Blank;
    } else {
        match word_vec[0] {
            "create" => {
                if word_vec.len() != 2 {
                    Command::Invalid("create requires degree argument".to_string())
                } else {
                    match word_vec[1].parse() {
                        Ok(degree) => Command::Create(degree),
                        Err(_) => Command::Invalid("degree must be (preferably small) integer".to_string()),
                    }
                }
            },
            "train" => {
                if word_vec.len() != 2 {
                    Command::Invalid("please specify path to train from".to_string())
                } else {
                    let mut p = PathBuf::new();
                    p.push(Path::new(word_vec[1]));
                    Command::Train(p)
                }
            },
            "generate" => {
                if word_vec.len() > 2 {
                    Command::Invalid("incorrect format for generate command".to_string())
                } else if word_vec.len() == 1 {
                    Command::Generate(1)
                } else {
                    match word_vec[1].parse() {
                        Ok(num) => Command::Generate(num),
                        Err(_) => Command::Invalid("number argument must be integer".to_string()),
                    }
                }
            }
            "exit" | "quit" | "exit()" => Command::Exit,
            _ => Command::Invalid("command not recognized".to_string())
        }
    }
                
}

#[derive(PartialEq, Eq, Debug)]
enum Command {
    Create(u32),
    Train(PathBuf),
    Invalid(String),
    Blank,
    Generate(u32),
    Exit,
}

