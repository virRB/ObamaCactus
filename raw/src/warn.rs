use colored::Colorize;
use std::process::exit;

pub fn warning(warn: &str) {
    println!("{}", warn.red());
    exit(1);
}
pub fn mistake(warn: &str) {
    println!("{}", warn.magenta());
    exit(1);
}