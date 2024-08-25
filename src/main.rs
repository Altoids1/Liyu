#![allow(non_snake_case)] //stfu Rust jeezus
//#![allow(dead_code)] // Remove this when we're mature :) 

use std::collections::VecDeque;
use std::io::{stdin, stdout, Write};

mod board;
mod test;
mod engine;
mod args;

use board::packedmove::{PackedCoord, PackedMove};

use crate::args::parseArgs;

/// An error handling layer above write! that generically handles all the Result<>s floating around.
#[macro_export]
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
    

    if parseArgs() { // When called successfully with arguments, we are not interactive.
        return;
    }

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
            "h" | "H" | "help" | "HELP" => {
                say!("Available commands:\n");
                say!("'fen [FenString]' - loads in a new position from a valid FEN string.\n");
                say!("'eval [Depth=6]' - returns the current evaluation of the position.\n");
                say!("'move [Move]' - plays the given move onto the last saved board\n");
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
                    1 => depth = 6,
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
            "move" | "MOVE" => {
                match words.len() {
                    1 => {say!("No move given to the 'move' command");},
                    2 => {
                        // Expecting to move to be in the pattern:
                        // a1b1
                        let moveStr = words[1];
                        if moveStr.len() != 4 {
                            say!("Move is invalid or in an unimplemented format");
                            continue;
                        }

                        let startCoord = PackedCoord::new_from_usize(
                            (moveStr.bytes().nth(1).unwrap() - b'1').into(),
                            (moveStr.bytes().nth(0).unwrap() - b'a').into()
                        );
                        let endCoord = PackedCoord::new_from_usize(
                            (moveStr.bytes().nth(3).unwrap() - b'1').into(),
                            (moveStr.bytes().nth(2).unwrap() - b'a').into()
                        );

                        let packedMove = PackedMove::new_from_packed(startCoord, endCoord);
                        say!("Move {} accepted.",packedMove);
                        boardPosition = boardPosition.branch(packedMove);
                        
                    },
                    _ => {say!("Too many arguments given to 'move' command");}
                }
            }
            "d" | "D" | "display" | "DISPLAY" => {
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