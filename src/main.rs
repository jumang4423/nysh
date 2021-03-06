/*
 * nyu shell by jumang
 * a cute shell
 * that written in rust
 */

// - objects
mod nysh_builtin;
mod tools;
// - generic input output thingy
use std::env;
use std::io;
use std::io::*;
use std::process::exit;
// - Emoji receiver usage
use rand::thread_rng;
use rand::Rng;
use colored::*;

// - main func
#[tokio::main]
async fn main() -> io::Result<()> {
    // check nysh library is available on computer
    match tools::fetch_lib::check_nysh_lib() {
        Some(_nysh_path) => {
            // nysh lib is not installed
            println!(
                "\n{}",
                "-! oi! nysh resources is missing😭"
                    .on_truecolor(150, 50, 50)
                    .white()
            );
            match tools::fetch_lib::auto_install_lib() {
                Ok(_) => {
                    println!(
                        "{}\n",
                        "-o ready to use nysh!"
                            .on_truecolor(50, 150, 50)
                            .white()
                    );
                }
                Err(_e) => {
                    println!("\n{}", "-! installation skipped".on_truecolor(255, 0, 0));
                    println!("{}\n", _e);
                }
            }
        }
        None => {
            //ignoreing
        }
    }

    print!("\x1B[2J\x1B[1;1H");

    tools::welcome_uis::logo_display().unwrap_or_else(|err: std::io::Error| {
        eprintln!("IO error => {}", err);
        exit(1);
    });

    println!();

    // - say hi!
    tools::welcome_uis::say_welcome().unwrap_or_else(|err: std::io::Error| {
        eprintln!("IO error => {}", err);
        exit(1);
    });
    tools::welcome_uis::logined_as().unwrap_or_else(|err: std::io::Error| {
        eprintln!("IO error => {}", err);
        exit(1);
    });

    // shell loops
    {
        nysh_letsgooooooo().await;
    }
    Ok(())
}

pub async fn nysh_letsgooooooo() {
    // - vars
    // drawing emoji
    let mut rng = thread_rng();
    // - shell loops
    loop {
        // - cummand waiter
        // get current directory
        let _current_path_chr = env::current_dir().unwrap();
        let current_path: String;
        // shorted path Option<String> goes here
        match tools::command_uis::path_abbr(String::from(_current_path_chr.to_str().unwrap())) {
            Ok(path) => current_path = path,
            Err(_) => {
                eprintln!("path error => ?path");
                exit(1);
            }
        }
        let _emoji_keys: usize = rng.gen_range(0..7);
        print!(
            "{} {} ",
            current_path,
            tools::command_uis::get_emoji(_emoji_keys),
        );

        stdout().flush().unwrap();

        // listen to the user input
        let mut waiter = String::new();

        waiter = tools::input::input(waiter);

        // parse command into command object
        let mut commands = tools::parser::CommandParser::constructor(waiter);
        commands.parse_it();

        let mut runner = tools::runner::CommandRunner::constructor(commands).unwrap();
        runner.run_command().await;
    }
}
