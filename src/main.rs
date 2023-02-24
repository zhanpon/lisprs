use std::str::FromStr;

#[derive(PartialEq)]
enum Atom {
    Symbol(String),
    Integer(i64),
}

enum SExpr {
    SList(Vec<Atom>),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSExprError;

impl FromStr for SExpr {
    type Err = ParseSExprError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let atoms = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .map(|s| s.split_whitespace())
            .ok_or(ParseSExprError)?;

        Ok(SExpr::SList(atoms.map(parse_atom).collect()))
    }
}

fn parse_atom(s: &str) -> Atom {
    match s.parse::<i64>() {
        Ok(i) => Atom::Integer(i),
        Err(_) => Atom::Symbol("+".to_string()),
    }
}

fn add_atoms(atoms: &[Atom]) -> i64 {
    let mut sum = 0;

    for x in atoms {
        if let Atom::Integer(i) = x {
            sum += i;
        } else {
            panic!()
        }
    }
    sum
}

fn eval(expr: SExpr) -> i64 {
    let SExpr::SList(expr) = expr;
    if expr[0] == Atom::Symbol("+".to_string()) {
        add_atoms(&expr[1..])
    } else {
        panic!()
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
}
