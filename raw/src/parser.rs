use std::collections::HashMap;
use crate::warn;
use crate::builtins;

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    Text(String),
    Boolean(bool),
    Nothing,
}

fn handle_function(line: &str, variables: &HashMap<String, Value>, builtins: &HashMap<String, Builtin>) -> Value {
    let name = get_function_name(line);
    let start = line.find('(').unwrap();
    let end = line.rfind(')').unwrap();
    let inner = &line[start + 1..end];
    let params = get_inner_params(inner);
    let mut args = Vec::new();
    for param in params {
        args.push(parse_value(&param, variables, builtins));
    }
    if let Some(function) = builtins.get(&name) {
        return function(args);
    } else {
        warn::warning(&format!("Unknown function '{}'", name));
        return Value::Nothing;
    }
}

fn get_inner_params(brackets: &str) -> Vec<String> {
    let mut in_reg_string = false;
    let mut in_curl_string = false;
    let mut paren_depth = 0;
    let mut result: Vec<String> = Vec::new();
    let mut so_far = String::new();
    for c in brackets.chars() {
        if c == '"' {
            if in_curl_string {
                so_far.push(c);
                continue;
            }
            in_reg_string = !in_reg_string;
            so_far.push(c);
            continue;
        }
        if c == '“' {
            if in_reg_string {
                warn::warning("Cannot start new string inside string");
                return Vec::new();
            }
            in_curl_string = true;
            so_far.push(c);
            continue;
        }
        if c == '”' {
            if !in_curl_string {
                warn::warning("No string to end");
                return Vec::new();
            }
            in_curl_string = false;
            so_far.push(c);
            continue;
        }
        if c == '(' && !in_reg_string && !in_curl_string {
            paren_depth += 1;
            so_far.push(c);
            continue;
        }
        if c == ')' && !in_reg_string && !in_curl_string {
            if paren_depth == 0 {
                warn::warning("Unexpected closing parenthesis");
                return Vec::new();
            }
            paren_depth -= 1;
            so_far.push(c);
            continue;
        }
        if c == ',' && !in_reg_string && !in_curl_string && paren_depth == 0 {
            result.push(so_far.trim().to_string());
            so_far.clear();
            continue;
        }
        so_far.push(c);
    }
    if in_reg_string || in_curl_string {
        warn::warning("Unclosed string");
        return Vec::new();
    }
    if paren_depth != 0 {
        warn::warning("Unclosed parenthesis");
        return Vec::new();
    }
    if !so_far.trim().is_empty() {
        result.push(so_far.trim().to_string());
    }
    return result;
}

fn get_function_name(brackets: &str) -> String {
    let mut result = String::new();
    for c in brackets.chars() {
        if c == '(' {
            return result;
        }
        result.push(c);
    }
    return result;
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Text(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nothing => write!(f, "Nothing"),
        }
    }
}

fn is_int(thing: &str) -> bool {
    match thing.parse::<i32>() {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub type Builtin = fn(Vec<Value>) -> Value;


fn parse_value(value: &str, variables: &HashMap<String, Value>, builtins: &HashMap<String, Builtin>) -> Value {
    if value.contains("(") && value.ends_with(")") {
        return handle_function(value, variables, builtins);
    } else if value.starts_with('"') && value.ends_with('"') {
        return Value::Text(value.strip_prefix('"').unwrap().strip_suffix('"').unwrap().to_string());
    } else if value.starts_with('“') && value.ends_with('”') {
        return Value::Text(value.strip_prefix('“').unwrap().strip_suffix('”').unwrap().to_string());
    } else if is_int(value) {
        return Value::Number(value.parse::<f64>().unwrap());
    } else if value == "yes" {
        return Value::Boolean(true);
    } else if value == "no" {
        return Value::Boolean(false);
    } else if variables.contains_key(value) {
        return variables[value].clone();
    } else {
        warn::warning(&format!("Unexpected value {}", value));
        return Value::Nothing;
    }
}

fn handle_var(statement: &str, variables: &mut HashMap<String, Value>, builtins: &HashMap<String, Builtin>) {
    let statement: Vec<&str> = statement.split("to").collect();
    if statement.len() != 2 {
        warn::warning("Expected key-val pair seperated by 'to'");
    }
    let key = statement[0].trim().to_string();
    let val = parse_value(statement[1].trim(), variables, builtins);
    variables.insert(key, val);
}

fn handle_change(statement: &str, variables: &mut HashMap<String, Value>, builtins: &HashMap<String, Builtin>) {
    let statement: Vec<&str> = statement.split("to").collect();
    if statement.len() != 2 {
        warn::warning("Expected key-val pair separated by 'to'");
    }
    let key = statement[0].trim();
    if !variables.contains_key(key) {
        warn::warning(&format!("Variable '{}' does not exist", key));
    }
    let val = parse_value(statement[1].trim(), variables, builtins);
    variables.insert(key.to_string(), val);
}

pub fn parse(line: &str, variables: &mut HashMap<String, Value>, builtins: &HashMap<String, Builtin>) {
    if line.starts_with("set") {
        let line = line.strip_prefix("set").unwrap().trim();
        handle_var(line, variables, builtins);
    } else if line.contains("(") && line.ends_with(")") {
        handle_function(line, variables, builtins);
    } else if line.starts_with("say") {
        let line = line.strip_prefix("say").unwrap().trim();
        println!("{}", parse_value(line, variables, builtins));
    } else if line.starts_with("change") {
        let line = line.strip_prefix("change").unwrap().trim();
        handle_change(line, variables, builtins);
    } else {
        warn::warning(&format!("Invalid statement: {}", line));
    }
}