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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Procedure(_) => todo!(),
        }
    }
}

fn sum_values(vs: &[Value]) -> Value {
    let mut acc = 0;
    for v in vs {
        match v {
            Value::Integer(n) => acc += n,
            Value::Procedure(_) => todo!(),
        }
    }

    Value::Integer(acc)
}

fn product_values(vs: &[Value]) -> Value {
    let mut acc = 1;
    for v in vs {
        match v {
            Value::Integer(n) => acc *= n,
            Value::Procedure(_) => todo!(),
        }
    }

    Value::Integer(acc)
}

fn eval_atom(atom: &Atom) -> Value {
    match atom {
        Atom::Integer(i) => Value::Integer(*i),
        Atom::Symbol(s) if s == "+" => Value::Procedure(Procedure::Sum),
        Atom::Symbol(s) if s == "*" => Value::Procedure(Procedure::Product),
        _ => todo!(),
    }
}

fn apply_procedure(proc: &Value, args: &[Value]) -> Value {
    match proc {
        Value::Procedure(Procedure::Sum) => sum_values(args),
        Value::Procedure(Procedure::Product) => product_values(args),
        _ => todo!(),
    }
}

pub fn eval(expr: &SExpr) -> Value {
    match expr {
        SExpr::Atom(a) => eval_atom(a),
        SExpr::SList(slist) => {
            let values: Vec<Value> = slist.iter().map(eval).collect();
            apply_procedure(&values[0], &values[1..])
        }
    }
}
