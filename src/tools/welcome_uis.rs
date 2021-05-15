// OverAllTools Object //

// - libraries
use colored::*;
use std::io;
use whoami;

// - welcome func
pub fn say_welcome() -> Result<(), io::Error> {
    let blue_color_res: Result<Color, ()> = "magenta".parse();
    println!(
        "-> welcome to nysh! is the shell written in {}",
        "rust".color(blue_color_res.unwrap_or(Color::Blue)).bold(),
    );
    Ok(())
}

// - login name func
pub fn logined_as() -> Result<(), io::Error> {
    let blue_color_res: Result<Color, ()> = "magenta".parse();
    println!(
        "-! logined as {}",
        whoami::username()
            .color(blue_color_res.unwrap_or(Color::Blue))
            .bold()
    );
    Ok(())
}