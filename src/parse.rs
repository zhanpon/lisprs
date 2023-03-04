use std::iter::Peekable;
use std::str::FromStr;

use crate::tokenize::Tokenizer;

#[derive(Debug, PartialEq)]
pub enum Atom {
    Symbol(String),
    Integer(i64),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum SExpr {
    Atom(Atom),
    SList(Vec<SExpr>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseSExprError;

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

pub struct Parser<'a> {
    pub tokenizer: Peekable<Tokenizer<'a>>,
}

impl<'a> Parser<'a> {
    fn consume_token(&mut self, expected: &str) -> Result<(), ParseSExprError> {
        match self.tokenizer.next() {
            Some(t) if t == expected => Ok(()),
            _ => Err(ParseSExprError),
        }
    }

    pub fn parse_expr(&mut self) -> Result<SExpr, ParseSExprError> {
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
