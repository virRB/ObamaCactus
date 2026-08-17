#![allow(warnings)]
use std::env;
use std::path::Path;
use std::fs;
use std::collections::HashMap;
mod parser;
mod warn;
mod builtins;
mod read_page;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        warn::mistake("Usage: obama -r <.obama file or .png file>");
    }
    if args[1] != "-r" && args[1] != "run" {
        warn::mistake("Unknown command. Usage: obama -r <.obama file or .png file>");
    }
    if args.len() < 3 {
        warn::mistake("Usage: obama -r <.obama file or .png file>");
    }
    let program = &args[2];

    if !program.ends_with(".obama") && !program.ends_with(".png") {
        warn::mistake("Program must be a .obama or .png file");
    }
    let source: String;
    if program.ends_with(".png") {
        if !Path::new(program).exists() {
            warn::warning(&format!("Cannot find file {}", program));
        }
        source = read_page::compile_paper_sheet(program)
            .unwrap_or_else(|err| {
                warn::warning(&format!("Failed to compile image: {}", err));
                String::new()
            });
    } else {
        if !Path::new(program).exists() {
            warn::warning(&format!("Cannot find file {}", program));
        }
        source = fs::read_to_string(program)
            .unwrap_or_else(|err| {
                warn::warning(&format!("Failed to read file: {}", err));
                String::new()
            });
    }
    let mut variables: HashMap<String, parser::Value> = HashMap::new();
    let builtins_ = builtins::get_builtins();
    for line in source.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("/") {
            continue;
        }
        let rest = if line.contains("/") {
            line.split("/").next().unwrap().trim()
        } else {
            line
        };
        parser::parse(rest, &mut variables, &builtins_);
    }
}