use crate::parser::Value;
use std::io;
use crate::warn;

pub fn out(args: Vec<Value>) -> Value {
    for arg in args {
        println!("{}", arg);
    }
    return Value::Nothing;
}

pub fn r#in(_: Vec<Value>) -> Value {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    return Value::Text(input);
}

pub fn sum(args: Vec<Value>) -> Value {
    let mut result = 0.0;
    for arg in args {
        match arg {
            Value::Number(n) => result += n,
            _ => warn::warning("sum() expects numbers"),
        }
    }
    return Value::Number(result);
}

pub fn sub(args: Vec<Value>) -> Value {
    let mut result = 0.0;
    for (i, arg) in args.iter().enumerate() {
        match arg {
            Value::Number(n) => {
                if i == 0 {
                    result = *n;
                    continue;
                }
                result -= n;
            }
            _ => warn::warning("sub() expects numbers"),
        }
    }
    return Value::Number(result);
}

pub fn mult(args: Vec<Value>) -> Value {
    let mut result = 1.0;
    for arg in args {
        match arg {
            Value::Number(n) => result *= n,
            _ => warn::warning("mult() expects numbers"),
        }
    }
    Value::Number(result)
}