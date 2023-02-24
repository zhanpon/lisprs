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

impl FromStr for SExpr {
    type Err = ParseSExprError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let atoms = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .map(|s| s.split_whitespace())
            .ok_or(ParseSExprError)?;

        atoms
            .into_iter()
            .map(|a| a.parse::<Atom>().map(SExpr::Atom))
            .collect::<Result<Vec<SExpr>, _>>()
            .map(SExpr::SList)
    }
}

fn add_atoms(atoms: &[SExpr]) -> i64 {
    let mut sum = 0;

    for x in atoms {
        if let SExpr::Atom(Atom::Integer(i)) = x {
            sum += i;
        } else {
            panic!()
        }
    }
    sum
}

fn mul_atoms(atoms: &[SExpr]) -> i64 {
    let mut sum = 1;

    for x in atoms {
        if let SExpr::Atom(Atom::Integer(i)) = x {
            sum *= i;
        } else {
            panic!()
        }
    }
    sum
}

fn eval(expr: SExpr) -> i64 {
    match expr {
        SExpr::Atom(_) => panic!(),
        SExpr::SList(slist) => match &slist[0] {
            SExpr::Atom(Atom::Symbol(s)) => match s.as_str() {
                "+" => add_atoms(&slist[1..]),
                "*" => mul_atoms(&slist[1..]),
                _ => panic!(),
            },
            _ => panic!(),
        },
    }
}

fn main() {
    let expr = "(+ 1 2)".parse().unwrap();
    let result = eval(expr);

    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(eval("(+ 2 3)".parse().unwrap()), 5);
        assert_eq!(eval("(+ 4 5)".parse().unwrap()), 9);
    }

    #[test]
    fn test_mul() {
        assert_eq!(eval("(* 2 3)".parse().unwrap()), 6);
        assert_eq!(eval("(* 4 5)".parse().unwrap()), 20);
    }

    #[test]
    fn test_parse_error() {
        assert_eq!(")".parse::<SExpr>(), Err(ParseSExprError))
    }
}
