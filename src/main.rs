fn parse(s: &str) -> Vec<&str> {
    return s
        .strip_prefix("(")
        .unwrap()
        .strip_suffix(")")
        .unwrap()
        .split_whitespace()
        .collect();
}

fn eval(s: Vec<&str>) -> i64 {
    if s[0] == "+" {
        return s[1..].iter().map(|a| a.parse::<i64>().unwrap()).sum();
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
