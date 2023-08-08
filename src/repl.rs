use crate::parser::{eval, tokenize, Env};
use std::io::prelude::*;

pub fn run() -> Result<(), String> {
    println!("Ctrl+C to exit.\n");

    let mut env = Env::new();
    let mut count = 1;

    loop {
        prompt(&mut count);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let result = eval(&mut tokenize(input.trim()), &mut env)?;
        println!("{result}");
    }
}

fn prompt(count: &mut i32) {
    print!("[{count}]> ");
    std::io::stdout().flush().unwrap();
    *count += 1;
}
