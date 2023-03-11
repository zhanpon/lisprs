use std::io::{stdin, stdout, Write};

use crate::parse::Parser;
use crate::parse::SExpr;
use crate::parse::{Atom, ParseSExprError};
use crate::tokenize::Tokenizer;

mod parse;
mod tokenize;

#[derive(Debug, PartialEq)]
enum Value {
    Integer(i64),
}

fn sum_values(vs: &[Value]) -> Value {
    let mut acc = 0;
    for v in vs {
        match v {
            Value::Integer(n) => acc += n,
        }
    }

    Value::Integer(acc)
}

fn product_values(vs: &[Value]) -> Value {
    let mut acc = 1;
    for v in vs {
        match v {
            Value::Integer(n) => acc *= n,
        }
    }

    Value::Integer(acc)
}

fn eval_atom(atom: &Atom) -> Value {
    match atom {
        Atom::Integer(i) => Value::Integer(*i),
        _ => todo!(),
    }
}

fn eval(expr: &SExpr) -> Value {
    match expr {
        SExpr::Atom(a) => eval_atom(a),
        SExpr::SList(slist) => match &slist[0] {
            SExpr::Atom(Atom::Symbol(s)) => match s.as_str() {
                "+" => sum_values(&slist[1..].iter().map(eval).collect::<Vec<Value>>()),
                "*" => product_values(&slist[1..].iter().map(eval).collect::<Vec<Value>>()),
                _ => todo!(),
            },
            _ => todo!(),
        },
    }
}

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

    fn parse_eval_unwrap(s: &str) -> Value {
        parse_eval(s).unwrap()
    }

    #[test]
    fn test_add() {
        assert_eq!(parse_eval_unwrap("(+ 2 3)"), Value::Integer(5));
        assert_eq!(parse_eval_unwrap("(+ 4 5)"), Value::Integer(9));
    }

    #[test]
    fn test_mul() {
        assert_eq!(parse_eval_unwrap("(* 2 3)"), Value::Integer(6));
        assert_eq!(parse_eval_unwrap("(* 4 5)"), Value::Integer(20));
    }

    #[test]
    fn test_atom() {
        assert_eq!(parse_eval_unwrap("3"), Value::Integer(3));
    }

    #[test]
    fn test_nested() {
        assert_eq!(parse_eval_unwrap("(+ 1 (* 2 3))"), Value::Integer(7));
        assert_eq!(
            parse_eval_unwrap("(+ (* 1 2) (* 3 (+ 4 5)))"),
            Value::Integer(29)
        );
    }
}
