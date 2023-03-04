mod tokenize;

use crate::tokenize::Tokenizer;
use std::str::FromStr;

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

fn consume_token<'a>(
    tokens: &mut impl Iterator<Item = &'a str>,
    expected: &str,
) -> Result<(), ParseSExprError> {
    match tokens.next() {
        Some(t) if t == expected => Ok(()),
        _ => Err(ParseSExprError),
    }
}

fn parse_slist(s: &str) -> Result<SExpr, ParseSExprError> {
    let mut tokens = Tokenizer::from(s);
    consume_token(&mut tokens, "(")?;

    let mut tokens: Vec<&str> = tokens.collect();

    let last_token = tokens.pop().ok_or(ParseSExprError)?;
    if last_token != ")" {
        return Err(ParseSExprError);
    }

    tokens
        .into_iter()
        .map(SExpr::from_str)
        .collect::<Result<Vec<SExpr>, _>>()
        .map(SExpr::SList)
}

impl FromStr for SExpr {
    type Err = ParseSExprError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('(') {
            parse_slist(s)
        } else {
            Atom::from_str(s).map(SExpr::Atom)
        }
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
    let ast = SExpr::from_str(s).unwrap();
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
    fn test_parse_error() {
        assert_eq!("(".parse::<SExpr>(), Err(ParseSExprError))
    }
}
