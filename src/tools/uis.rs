// OverAllTools Object //

// - libraries
use colored::*;
use console::Emoji;
use std::io;
use whoami;

// - welcome func
pub fn say_welcome() -> Result<(), io::Error> {
    let blue_color_res: Result<Color, ()> = "magenta".parse();
    println!(
        "{} welcome to nysh, the shell written in {}",
        Emoji("🐶", ""),
        "RUST".color(blue_color_res.unwrap_or(Color::Blue)).bold(),
    );
    Ok(())
}

// - login name func
pub fn logined_as() -> Result<(), io::Error> {
    let blue_color_res: Result<Color, ()> = "magenta".parse();
    println!(
        "-> logined as {}",
        whoami::username()
            .color(blue_color_res.unwrap_or(Color::Blue))
            .bold()
    );
    Ok(())
}

// - show current directory func
pub fn path_abbr(path: String) -> Result<String, io::Error> {
    // string to splitted by / and translates into Option<char> iterator
    let current_folder: &str;
    match path.split('/').last() {
        Some(d) => current_folder = d,
        None => current_folder = "invalid",
    }

    let mut _path: Vec<String> = path
        .split('/')
        // filter nothing char
        .filter(|_temp| match _temp.chars().next() {
            Some(_) => true,
            None => false,
        })
        // filter nothing &str to char
        .map(|_temp| match _temp.chars().next() {
            Some(d) => {
                return String::from(d);
            }
            None => return String::from(""),
        })
        .collect();

    // concating current directory and others on really dirty way
    let mut _str: String;
    _path.pop();
    _str = _path.join("/") + "/" + current_folder;

    Ok(_str)
}
