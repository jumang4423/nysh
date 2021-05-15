use std::process::exit;

pub fn builtin_exit() {
    println!("-! killing process...");
    exit(1);
}