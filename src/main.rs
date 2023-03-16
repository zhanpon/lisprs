use crate::eval::{eval, Value};
use std::io::{stdin, stdout, Write};

use crate::parse::ParseSExprError;
use crate::parse::Parser;
use crate::tokenize::Tokenizer;

mod eval;
mod parse;
mod tokenize;

fn parse_eval(s: &str) -> Result<Value, ParseSExprError> {
    let tokens = Tokenizer::from(s);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_expr()?;
    Ok(eval(&ast))
}

fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input_string = String::new();
        stdin().read_line(&mut input_string).unwrap();

        if input_string.is_empty() {
            break;
        } else if input_string == "\n" {
            continue;
        }

        match parse_eval(input_string.trim()) {
            Ok(v) => println!("{:?}", v),
            Err(e) => println!("{}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_evaluates_to(expr: &str, value: i64) {
        let result = parse_eval(expr).unwrap();
        assert_eq!(result, Value::Integer(value));
    }

    #[test]
    fn test_add() {
        assert_evaluates_to("(+ 2 3)", 5);
        assert_evaluates_to("(+ 4 5)", 9);
    }

    #[test]
    fn test_mul() {
        assert_evaluates_to("(* 2 3)", 6);
        assert_evaluates_to("(* 4 5)", 20);
    }

    #[test]
    fn test_atom() {
        assert_evaluates_to("3", 3);
    }

    #[test]
    fn test_nested() {
        assert_evaluates_to("(+ 1 (* 2 3))", 7);
        assert_evaluates_to("(+ (* 1 2) (* 3 (+ 4 5)))", 29);
    }
}
