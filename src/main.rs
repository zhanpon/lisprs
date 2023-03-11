use std::io::{stdin, stdout, Write};

use crate::parse::Parser;
use crate::parse::SExpr;
use crate::parse::{Atom, ParseSExprError};
use crate::tokenize::Tokenizer;

mod parse;
mod tokenize;

fn eval_atom(atom: &Atom) -> i64 {
    match atom {
        Atom::Integer(i) => *i,
        _ => todo!(),
    }
}

fn eval(expr: &SExpr) -> i64 {
    match expr {
        SExpr::Atom(a) => eval_atom(a),
        SExpr::SList(slist) => match &slist[0] {
            SExpr::Atom(Atom::Symbol(s)) => match s.as_str() {
                "+" => slist[1..].iter().map(eval).sum(),
                "*" => slist[1..].iter().map(eval).product(),
                _ => todo!(),
            },
            _ => todo!(),
        },
    }
}

fn parse_eval(s: &str) -> Result<i64, ParseSExprError> {
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

        match parse_eval(input_string.as_str()) {
            Ok(v) => println!("{}", v),
            Err(e) => println!("{:?}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_eval_unwrap(s: &str) -> i64 {
        parse_eval(s).unwrap()
    }

    #[test]
    fn test_add() {
        assert_eq!(parse_eval_unwrap("(+ 2 3)"), 5);
        assert_eq!(parse_eval_unwrap("(+ 4 5)"), 9);
    }

    #[test]
    fn test_mul() {
        assert_eq!(parse_eval_unwrap("(* 2 3)"), 6);
        assert_eq!(parse_eval_unwrap("(* 4 5)"), 20);
    }

    #[test]
    fn test_atom() {
        assert_eq!(parse_eval_unwrap("3"), 3);
    }

    #[test]
    fn test_nested() {
        assert_eq!(parse_eval_unwrap("(+ 1 (* 2 3))"), 7);
        assert_eq!(parse_eval_unwrap("(+ (* 1 2) (* 3 (+ 4 5)))"), 29);
    }
}
