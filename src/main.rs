use crate::parse::Atom;
use crate::parse::Parser;
use crate::parse::SExpr;
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
