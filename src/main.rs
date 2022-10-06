use std::env;

fn main() { 
    let args: Vec<String> = env::args().collect(); // First entry is always just like.. a relative path to where we are?
    //dbg!(args);
    println!("Liyu - Version {}",env!("CARGO_PKG_VERSION"));
}