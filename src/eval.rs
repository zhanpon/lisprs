use crate::parse::Atom;
use crate::parse::SExpr;

#[derive(Debug, PartialEq)]
pub enum Value {
    Integer(i64),
}

fn sum_values(vs: &[Value]) -> Value {
    let mut acc = 0;
    for v in vs {
        match v {
            Value::Integer(n) => acc += n,
        }
    }

    Value::Integer(acc)
}

fn product_values(vs: &[Value]) -> Value {
    let mut acc = 1;
    for v in vs {
        match v {
            Value::Integer(n) => acc *= n,
        }
    }

    Value::Integer(acc)
}

fn eval_atom(atom: &Atom) -> Value {
    match atom {
        Atom::Integer(i) => Value::Integer(*i),
        _ => todo!(),
    }
}

pub fn eval(expr: &SExpr) -> Value {
    match expr {
        SExpr::Atom(a) => eval_atom(a),
        SExpr::SList(slist) => match &slist[0] {
            SExpr::Atom(Atom::Symbol(s)) => match s.as_str() {
                "+" => sum_values(&slist[1..].iter().map(eval).collect::<Vec<Value>>()),
                "*" => product_values(&slist[1..].iter().map(eval).collect::<Vec<Value>>()),
                _ => todo!(),
            },
            _ => todo!(),
        },
    }
}
