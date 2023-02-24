#[derive(PartialEq)]
enum Atom {
    Symbol(String),
    Integer(i64),
}

fn parse_atom(s: &str) -> Atom {
    match s.parse::<i64>() {
        Ok(i) => Atom::Integer(i),
        Err(_) => Atom::Symbol("+".to_string()),
    }
}

fn parse(s: &str) -> Vec<Atom> {
    return s
        .strip_prefix("(")
        .unwrap()
        .strip_suffix(")")
        .unwrap()
        .split_whitespace()
        .map(parse_atom)
        .collect();
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
    return sum;
}

fn eval(s: Vec<Atom>) -> i64 {
    if s[0] == Atom::Symbol("+".to_string()) {
        return add_atoms(&s[1..]);
    } else {
        panic!()
    }
}

fn main() {
    let s = "(+ 1 2)";
    let expr = parse(s);
    let result = eval(expr);

    println!("{:?}", result);
}

mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(eval(parse("(+ 2 3)")), 5);
        assert_eq!(eval(parse("(+ 4 5)")), 9);
    }
}
