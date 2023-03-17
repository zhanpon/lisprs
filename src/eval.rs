use crate::parse::Atom;
use crate::parse::SExpr;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Procedure {
    Sum,
    Product,
}

#[derive(Debug, PartialEq)]
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
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvalError::ContractViolation => write!(f, "contract violation"),
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

fn eval_atom(atom: &Atom) -> Value {
    match atom {
        Atom::Integer(i) => Value::Integer(*i),
        Atom::Symbol(s) if s == "+" => Value::Procedure(Procedure::Sum),
        Atom::Symbol(s) if s == "*" => Value::Procedure(Procedure::Product),
        _ => todo!(),
    }
}

fn apply_procedure(proc: &Value, args: &[Value]) -> Result<Value, EvalError> {
    match proc {
        Value::Procedure(Procedure::Sum) => sum_values(args),
        Value::Procedure(Procedure::Product) => product_values(args),
        _ => todo!(),
    }
}

pub fn eval(expr: &SExpr) -> Result<Value, EvalError> {
    match expr {
        SExpr::Atom(a) => Ok(eval_atom(a)),
        SExpr::SList(slist) => {
            let values: Vec<Value> = slist.iter().map(|e| eval(e).unwrap()).collect();
            apply_procedure(&values[0], &values[1..])
        }
    }
}
