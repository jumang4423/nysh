use colored::*;

pub fn builtin_help() -> Result<(), String> {
    println!("");

    println!("-> available built-in commands:");
    println!("-! {}: change directory", "cd".magenta());
    println!("-! {}: simple ls", "nsd".magenta());
    println!("-! {}: ls but shortcut of nsd", "l".magenta());
    println!("-! {}: ls -al shortcut of nsd", "ll".magenta());
    println!("-! {}: cd ..", "..".magenta());
    println!("-! {}: make a secret contents locker", "dream95".magenta());
    println!(
        "-! {}: displays given image on terminal directly",
        "nywer".magenta()
    );
    println!("-! {}: help nysh", "help".magenta());
    println!("-! {}: exit nysh", "exit".magenta());

    println!("");
    Ok(())
}
