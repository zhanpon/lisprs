use crate::eval::Env;
use crate::eval::{eval, Value};
use std::io::{stdin, stdout, Write};

mod eval;
mod parse;
mod tokenize;

fn parse_eval(s: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let env = Env::empty();
    let ast = s.parse()?;
    Ok(eval(&ast, &env)?)
}

fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input_string = String::new();
        stdin().read_line(&mut input_string).unwrap();

        // Ctrl+D
        if input_string.is_empty() {
            break;
        // Enter
        } else if input_string == "\n" {
            continue;
        }

        match parse_eval(input_string.trim()) {
            Ok(v) => println!("{}", v),
            Err(e) => println!("{}", e),
        }
    }
}
