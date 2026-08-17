use std::collections::HashMap;
use crate::parser::{Builtin, Value};

pub mod standard;

pub fn get_builtins() -> HashMap<String, Builtin> {
    let mut builtins: HashMap<String, Builtin> = HashMap::new();
    builtins.insert("out".to_string(), standard::out);
    builtins.insert("in".to_string(), standard::r#in);
    builtins.insert("sum".to_string(), standard::sum);
    builtins.insert("sub".to_string(), standard::sub);
    builtins.insert("mult".to_string(), standard::mult);
    return builtins;
}