use colored::Colorize;
use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::{self, File};
use std::io;
pub fn get_emoji(emoji_number: usize) -> String {
    let emojis = ["💓", "👀", "🚗", "💰", "🍔", "🌸", "🐙"];
    String::from(emojis[emoji_number])
}

// // - show current directory func
// pub fn path_abbr_original(path: String) -> Result<String, io::Error> {
//     // string to splitted by / and translates into Option<char> iterator
//     let current_folder: &str;
//     match path.split('/').last() {
//         Some(d) => current_folder = d,
//         None => current_folder = "invalid",
//     }

//     let mut _path: Vec<String> = path
//         .split('/')
//         // filter nothing char
//         .filter(|_temp: &&str| match _temp.chars().next() {
//             Some(_) => true,
//             None => false,
//         })
//         // filter nothing &str to char
//         .map(|_temp| match _temp.chars().next() {
//             Some(d) => {
//                 return String::from(d);
//             }
//             None => return String::from(""),
//         })
//         .collect();

//     // concating current directory and others on really dirty way
//     let mut _str: String;
//     _path.pop();
//     _str = _path.join("/") + "/" + current_folder;

//     Ok(_str)
// }

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
        .filter(|_temp: &&str| match _temp.chars().next() {
            Some(_) => true,
            None => false,
        })
        // filter nothing &str to char
        .map(|_temp| match _temp.chars().next() {
            Some(_) => {
                return String::from(&_temp[0..1]); //&_temp[0.._temp.len() % 10 // original
            }
            None => return String::from(""),
        })
        .collect();

    // concating current directory and others on really dirty way
    let mut _str: String;
    _path.pop();

    _str = format!(
        "{} {}{} {}",
        (if _path.len() > 2 {
            "🏠".to_string()
        } else {
            "".to_string()
        }),
        (_path[2 % _path.len()..].join("/") + "/").truecolor(255, 100, 255),
        current_folder.bold().truecolor(239, 147, 255),
        check_folder_git()
    )
    .on_truecolor(24, 24, 24)
    .to_string();

    Ok(_str)
}
fn check_folder_git() -> String {
     
    let mut output=String::new();
    let mut paths = fs::read_dir(".").unwrap();
    let path = paths.find(|path| path.as_ref().unwrap().path() == PathBuf::from("./.git"));
    match path {
        Some(_) => {
            
            let mut git_head = File::open("./.git/HEAD").unwrap();
            let mut content = String::new();
            git_head.read_to_string(&mut content).unwrap();
            let refs: Vec<&str> = content.split("/").collect();
           output=format!("🐙 → {}" ,&refs[refs.len() - 1][..refs[refs.len() - 1].len()-1]);
        }
        None => {}
    };
    
    return  output;
}
