use std::{env, panic::catch_unwind};
use std::io::{stdout, Write};
use crate::say;

pub struct Argument {
    pub key : Option<String>,
    pub value : Option<String>
}

impl Argument {
    pub fn new(str : &String) -> Self {
        let mut ret : Self = Self {
            key : Default::default(),
            value : Default::default()
        };
        if !str.starts_with("--") {
            ret.value = Some(str.clone());
            return ret;
        }
        let strNoDash = String::from(str.get(2..).unwrap());
        if !str.contains('=') {
            ret.key = Some(strNoDash);
            return ret;
        }
        let sliceSplit : Vec<&str> = strNoDash.split("=").collect();
        ret.key = Some(sliceSplit[0].to_string());
        if sliceSplit.len() > 2 {
            ret.value = Some(sliceSplit[1..].join("=").to_string());
        }
        ret.value = Some(sliceSplit[1].to_string());
        return ret;
    }
}

pub fn parseArgs() -> bool {
    //Rust does a hard panic sometimes when the argument isn't proper UTF8, which is a tad ridiculous.
    //There's not really a simple way of avoiding that beyond what is happening below.
    let tryArgs : Result<Vec<String>,_> = catch_unwind(|| env::args().collect()); // First entry is always just like.. a relative path to where we are?
    if tryArgs.is_err() { // Invalid uniode, apparently.
        return false;
    }
    let args = tryArgs.ok().unwrap();
    if args.len() < 2 {
        return false;
    }
    let mut boardPosition : crate::board::BoardState = crate::board::BoardState::new();
    let mut depth = 6;
    for i in 1..args.len() {
        let arg = Argument::new(&args[i]);
        if arg.key.is_none() {
            return false;
        }
        match arg.key.as_ref().unwrap().as_str() {
            "fen" => {
                let fenstr = arg.value.unwrap();
                boardPosition = crate::board::BoardState::new_from_FEN(fenstr.as_str())
            }
            "depth" => {
                let depthstr = arg.value.unwrap();
                let depthResult = depthstr.parse::<i32>();
                if depthResult.is_err() {
                    say!("Invalid depth {}",depthstr);
                    return false;
                }
                depth = depthResult.ok().unwrap();
            }
            &_ => {
                say!("Unknown argument {}",arg.key.as_ref().unwrap());
                return false;
            }
        }
    }
    say!("{}\n",crate::engine::Engine::evalToDepth(&boardPosition, depth));


    return true;
}