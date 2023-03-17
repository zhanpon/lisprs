use std::fmt;
use std::iter::Peekable;
use std::str::FromStr;

use crate::tokenize::Tokenizer;

#[derive(Debug, PartialEq)]
pub enum Atom {
    Symbol(String),
    Integer(i64),
}

#[derive(Debug, PartialEq)]
pub enum SExpr {
    Atom(Atom),
    SList(Vec<SExpr>),
}

#[derive(Debug)]
pub enum ParseSExprError {
    UnmatchedParen,
}

impl fmt::Display for ParseSExprError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseSExprError::UnmatchedParen => write!(f, "expected a `)` to close `(`"),
        }
    }
}

impl std::error::Error for ParseSExprError {}

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

impl FromStr for SExpr {
    type Err = ParseSExprError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = Tokenizer::from(s);
        let mut parser = Parser::new(tokens);
        parser.parse_expr()
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

    fn consume_token(&mut self, expected: &str) {
        let token = self
            .tokenizer
            .next()
            .unwrap_or_else(|| panic!("Expected {}, but got no token", expected));

        if token != expected {
            panic!("Expected {}, but got {}", expected, token);
        }
    }

    fn parse_atom(&mut self) -> Result<Atom, ParseSExprError> {
        let token = self.tokenizer.next().unwrap();
        Atom::from_str(token)
    }

    pub fn parse_expr(&mut self) -> Result<SExpr, ParseSExprError> {
        let first_token = self.tokenizer.peek().unwrap();

        if first_token == &"(" {
            self.parse_slist().map(SExpr::SList)
        } else {
            self.parse_atom().map(SExpr::Atom)
        }
    }

    fn parse_slist(&mut self) -> Result<Vec<SExpr>, ParseSExprError> {
        self.consume_token("(");

        let mut exprs: Vec<SExpr> = vec![];

        while let Some(token) = self.tokenizer.peek() {
            if token == &")" {
                self.consume_token(")");
                return Ok(exprs);
            }

            exprs.push(self.parse_expr()?);
        }

        Err(ParseSExprError::UnmatchedParen)
    }
}
