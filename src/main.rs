#![allow(non_snake_case)] //stfu Rust jeezus
//#![allow(dead_code)] // Remove this when we're mature :) 

use std::collections::VecDeque;
use std::env;
use std::io::{stdin, stdout, Write};

mod board;
mod test;
mod engine;

/// An error handling layer above write! that generically handles all the Result<>s floating around.
macro_rules! say {
    ($($y:expr),+) => {
        let mut good = false;
        for _attempt in 1..=10 {
            if write!(stdout().lock(),$($y),+).is_ok() && stdout().flush().is_ok() {
                good = true;
                break;
            }
        }
        if !good {
            panic!("Writing failed after 10 tries");
        }
    }
}

fn main() { 
    let _args: Vec<String> = env::args().collect(); // First entry is always just like.. a relative path to where we are?

    println!("Liyu - Version {}",env!("CARGO_PKG_VERSION"));
    let mut boardPosition : board::BoardState = board::BoardState::new();
    loop {
        say!("\n> ");
        let mut cmdstr = String::new();
        _ = stdin().read_line(&mut cmdstr);

        let mut words = VecDeque::from_iter(cmdstr.split_ascii_whitespace());
        if words.len() == 0 {
            continue;
        }
        match words[0] {
            "h" | "help" | "HELP" => {
                say!("Available commands:\n");
                say!("'fen [FenString]' - loads in a new position from a valid FEN string.\n");
                say!("'eval [Depth=3]' - returns the current evaluation of the position.\n");
                say!("'display' - displays an ASCII depiction of the current board.\n");
                say!("'quit' - exits the program.");
            }
            "fen" | "FEN" => {
                words.pop_front();
                let fenstr = words.make_contiguous().join(" ");
                boardPosition = board::BoardState::new_from_FEN(fenstr.as_str());
                say!("Board position now: {}",boardPosition.writeFEN());
            }
            "eval" | "EVAL" => {
                let depth : i32;
                match words.len() {
                    1 => depth = 3,
                    2 => {
                        let cmd = words[1].parse::<i32>();
                        if cmd.is_err() {
                            say!("Invalid argument to 'eval' - argument must be integer");
                            continue;
                        }
                        depth = cmd.unwrap();
                    }
                    _ => {
                        say!("Too many arguments to 'eval'");
                        continue;
                    }
                }
                say!("Current evaluation: {}",engine::Engine::evalToDepth(&boardPosition, depth));
            }
            "display" | "DISPLAY" => {
                boardPosition.Display();
            }
            "quit" | "q" | "QUIT" => {
                break;
            }
            _ => {
                println!("Unrecognized command {}",words[0]);
                continue;
            }
        }
    };
}