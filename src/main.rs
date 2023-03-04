use std::iter::Peekable;
use std::str::FromStr;

use crate::tokenize::Tokenizer;

mod tokenize;

#[derive(Debug, PartialEq)]
enum Atom {
    Symbol(String),
    Integer(i64),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum SExpr {
    Atom(Atom),
    SList(Vec<SExpr>),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSExprError;

impl FromStr for Atom {
    type Err = ParseSExprError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let atom = match s.parse::<i64>() {
            Ok(i) => Atom::Integer(i),
            Err(_) => Atom::Symbol(s.to_string()),
        };

        Ok(atom)
    }
}

struct Parser<'a> {
    tokenizer: Peekable<Tokenizer<'a>>,
}

impl<'a> Parser<'a> {
    fn consume_token(&mut self, expected: &str) -> Result<(), ParseSExprError> {
        match self.tokenizer.next() {
            Some(t) if t == expected => Ok(()),
            _ => Err(ParseSExprError),
        }
    }

    fn parse_expr(&mut self) -> Result<SExpr, ParseSExprError> {
        let tokens = &mut self.tokenizer;
        let maybe_first_token = tokens.peek();

        if maybe_first_token.is_none() {
            return Err(ParseSExprError);
        }

        let first_token = maybe_first_token.unwrap();
        if first_token == &"(" {
            self.parse_slist()
        } else {
            Ok(SExpr::Atom(tokens.next().unwrap().parse().unwrap()))
        }
    }

    fn parse_slist(&mut self) -> Result<SExpr, ParseSExprError> {
        // let tokens = &mut self.tokenizer;
        self.consume_token("(")?;

        let mut exprs: Vec<SExpr> = vec![];

        loop {
            if self.tokenizer.peek().is_none() {
                return Err(ParseSExprError);
            }
            if self.tokenizer.peek() == Some(&")") {
                break;
            }

            let expr = self.parse_expr().unwrap();
            exprs.push(expr);
        }

        self.consume_token(")")?;
        Ok(SExpr::SList(exprs))
    }
}

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
                _ => panic!(),
            },
            _ => panic!(),
        },
    }
}

fn parse_eval(s: &str) -> i64 {
    let tokens = Tokenizer::from(s);
    let mut parser = Parser {
        tokenizer: tokens.peekable(),
    };
    let ast = parser.parse_expr().unwrap();
    eval(&ast)
}

fn main() {
    let result = parse_eval("(+ 1 2)");

    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(parse_eval("(+ 2 3)"), 5);
        assert_eq!(parse_eval("(+ 4 5)"), 9);
    }

    #[test]
    fn test_mul() {
        assert_eq!(parse_eval("(* 2 3)"), 6);
        assert_eq!(parse_eval("(* 4 5)"), 20);
    }

    #[test]
    fn test_atom() {
        assert_eq!(parse_eval("3"), 3);
    }

    #[test]
    fn test_nested() {
        assert_eq!(parse_eval("(+ 1 (* 2 3))"), 7);
    }
}
