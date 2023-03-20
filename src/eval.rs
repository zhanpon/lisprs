use crate::parse::Atom;
use crate::parse::SExpr;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Procedure {
    Sum,
    Product,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Integer(i64),
    Procedure(Procedure),
}

impl Value {
    fn as_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => Some(*i),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Procedure(_) => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum EvalError {
    ContractViolation,
    Undefined(String),
    BadSyntax(String),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalError::ContractViolation => write!(f, "contract violation"),
            EvalError::Undefined(x) => write!(f, "{}: undefined", x),
            EvalError::BadSyntax(x) => write!(f, "{}: bad syntax", x),
        }
    }
}

impl std::error::Error for EvalError {}

fn sum_values(vs: &[Value]) -> Result<Value, EvalError> {
    vs.iter()
        .map(|v| v.as_integer())
        .sum::<Option<i64>>()
        .map(Value::Integer)
        .ok_or(EvalError::ContractViolation)
}

fn product_values(vs: &[Value]) -> Result<Value, EvalError> {
    vs.iter()
        .map(|v| v.as_integer())
        .product::<Option<i64>>()
        .map(Value::Integer)
        .ok_or(EvalError::ContractViolation)
}

#[derive(Clone)]
pub struct Env {
    table: HashMap<String, Value>,
}

impl Env {
    pub fn standard() -> Self {
        let mut map = HashMap::new();
        map.insert("+".to_string(), Value::Procedure(Procedure::Sum));
        map.insert("*".to_string(), Value::Procedure(Procedure::Product));

        Self { table: map }
    }

    fn get(&self, k: &str) -> Option<&Value> {
        self.table.get(k)
    }

    fn with_binding(&self, s: String, v: Value) -> Self {
        let mut new_env: Env = self.clone();
        new_env.table.insert(s, v);
        new_env
    }
}

fn eval_atom(atom: &Atom, env: &Env) -> Result<Value, EvalError> {
    match atom {
        Atom::Integer(i) => Ok(Value::Integer(*i)),
        Atom::Symbol(s) => {
            if let Some(v) = env.get(s) {
                Ok(v.clone())
            } else {
                Err(EvalError::Undefined(s.clone()))
            }
        }
    }
}

fn apply_procedure(proc: &Value, args: &[Value]) -> Result<Value, EvalError> {
    match proc {
        Value::Procedure(Procedure::Sum) => sum_values(args),
        Value::Procedure(Procedure::Product) => product_values(args),
        _ => todo!(),
    }
}

fn parse_binding(binding: &SExpr) -> Result<(&str, &SExpr), EvalError> {
    let slist = binding
        .as_slist()
        .ok_or(EvalError::BadSyntax("let".to_string()))?;
    match slist.as_slice() {
        [SExpr::Atom(Atom::Symbol(s)), expr] => Ok((s, expr)),
        _ => Err(EvalError::BadSyntax("let".to_string())),
    }
}

fn parse_bindings(bindings: &Vec<SExpr>) -> Result<Vec<(&str, &SExpr)>, EvalError> {
    let mut result = Vec::new();
    for binding in bindings {
        result.push(parse_binding(binding)?)
    }
    Ok(result)
}

fn parse_let(slist: &Vec<SExpr>) -> Result<(Vec<(&str, &SExpr)>, &SExpr), EvalError> {
    match slist.as_slice() {
        [_, SExpr::SList(bindings), body] => Ok((parse_bindings(bindings)?, body)),
        _ => Err(EvalError::BadSyntax("let".to_string())),
    }
}

fn eval_let(slist: &Vec<SExpr>, env: &Env) -> Result<Value, EvalError> {
    let (bindings, body) = parse_let(slist)?;
    let mut new_env = env.clone();

    for (id, expr) in bindings {
        let v = eval(expr, env)?;
        new_env = new_env.with_binding(id.to_string(), v)
    }

    eval(body, &new_env)
}

pub fn eval(expr: &SExpr, env: &Env) -> Result<Value, EvalError> {
    match expr {
        SExpr::Atom(a) => eval_atom(a, env),
        SExpr::SList(slist) => match slist.as_slice() {
            [SExpr::Atom(Atom::Symbol(s)), ..] if s == "let" => eval_let(slist, env),
            _ => {
                let values: Vec<Value> = slist.iter().map(|e| eval(e, env).unwrap()).collect();
                apply_procedure(&values[0], &values[1..])
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_evaluates_to(expr: &str, value: i64) {
        let empty_env = Env::standard();
        let result = eval(&expr.parse().unwrap(), &empty_env).unwrap();
        assert_eq!(result, Value::Integer(value));
    }

    #[test]
    fn test_add() {
        assert_evaluates_to("(+ 2 3)", 5);
        assert_evaluates_to("(+ 4 5)", 9);
    }

    #[test]
    fn test_mul() {
        assert_evaluates_to("(* 2 3)", 6);
        assert_evaluates_to("(* 4 5)", 20);
    }

    #[test]
    fn test_atom() {
        assert_evaluates_to("3", 3);
    }

    #[test]
    fn test_nested() {
        assert_evaluates_to("(+ 1 (* 2 3))", 7);
        assert_evaluates_to("(+ (* 1 2) (* 3 (+ 4 5)))", 29);
    }

    #[test]
    fn test_let() {
        assert_evaluates_to("(let ((x 2)) (+ x 3))", 5);
        assert_evaluates_to("(let ((x 2) (y (* 3 4))) (+ x y))", 14);
    }

    #[test]
    fn test_eval_error() {
        let empty_env = Env::standard();
        let result = eval(&"(+ 2 *)".parse().unwrap(), &empty_env);
        assert!(result.is_err())
    }
}
