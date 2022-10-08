#![allow(non_snake_case)] //stfu Rust jeezus

use std::env;
mod board;

fn main() { 
    let args: Vec<String> = env::args().collect(); // First entry is always just like.. a relative path to where we are?
    //dbg!(args);
    println!("Liyu - Version {}",env!("CARGO_PKG_VERSION"));

    let starting_board : board::BoardState = board::BoardState::new();
    starting_board.Display();
}