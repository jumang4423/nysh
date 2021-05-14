/*
 * nysh shell by jumang
 * a really lightweight shell
 * that written in rust lang
 */

// - generic input output thingy
use std::env;
use std::io::*;
use std::process::exit;
// - welcome object
mod tools;

// - main func
fn main() {
    // say hi!
    tools::uis::say_welcome().unwrap_or_else(|err| {
        eprintln!("IO error => {}", err);
        exit(1);
    });
    tools::uis::logined_as().unwrap_or_else(|err| {
        eprintln!("IO error => {}", err);
        exit(1);
    });

    loop {
        // get current directory
        let _current_path_chr = env::current_dir().unwrap();
        let current_path: String;

        // shorted path Option<String> goes here
        match tools::uis::path_abbr(String::from(_current_path_chr.to_str().unwrap())) {
            Ok(path) => current_path = path,
            Err(_) => {
                eprintln!("path error => ?path");
                exit(1);
            }
        }
        // cummand waiter
        print!("{} ", current_path);
        stdout().flush().unwrap();

        // listen to the user input
        let mut waiter = String::new();
        stdin().read_line(&mut waiter).expect("input error => ?");
    }
}
