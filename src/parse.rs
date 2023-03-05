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
    pub fn new(tokenizer: Tokenizer) -> Parser {
        Parser {
            tokenizer: tokenizer.peekable(),
        }
    }

    fn consume_token(&mut self, expected: &str) -> Result<(), ParseSExprError> {
        match self.tokenizer.next() {
            Some(t) if t == expected => Ok(()),
            _ => Err(ParseSExprError),
        }
    }

    fn parse_atom(&mut self) -> Result<Atom, ParseSExprError> {
        let token = self.tokenizer.next().ok_or(ParseSExprError)?;
        Atom::from_str(token)
    }

    pub fn parse_expr(&mut self) -> Result<SExpr, ParseSExprError> {
        let first_token = self.tokenizer.peek().ok_or(ParseSExprError)?;

        if first_token == &"(" {
            self.parse_slist()
        } else {
            self.parse_atom().map(SExpr::Atom)
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
