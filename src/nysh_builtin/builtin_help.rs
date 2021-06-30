use colored::*;

pub fn builtin_help() -> Result<(), String> {

    println!("");

    println!("-> available built-in commands:");
    println!("-! {}: change directory", "cd".magenta());
    println!("-! {}: ls -al","la".magenta());
    println!("-! {}: cd ..","..".magenta());
    println!("-! {}: help nysh","help".magenta());
    println!("-! {}: exit nysh","exit".magenta());

    println!("");
    Ok(())
}