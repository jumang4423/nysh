use colored::*;
use std::env;
use std::fs;
use std::process::exit;

use std::ffi::OsStr;
use std::path::Path;

pub fn builtin_nsd(_args: Vec<String>) {
    let mut option: bool = false;

    if _args.is_empty() {
        // nsd
        option = false;
    } else if _args.len() == 1 {
        // nsd *
        if _args[0] == "l" {
            option = true;
        }
    } else {
        exit(1);
    }

    printer(option);
    println!();
}

pub fn printer(option: bool) {
    // where is the path
    let paths = fs::read_dir(&Path::new(&env::current_dir().unwrap())).unwrap();

    // header
    println!();
    if option {
        println!(
            "🐱 {0: <18} {1: <18} {2: <18}",
            "[sus]".green(),
            "[size]".green(),
            "[ext]".green()
        );
    } else {
        println!("🐱 {0: <18}", "[sus]".green(),);
    }

    // . ..
    println!("🐱 {0: <18} {1: <18}", ".", "");
    println!("🐱 {0: <18} {1: <18}", "..", "");

    let names = paths
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
            })
        })
        .collect::<Vec<String>>();

    for path_string in names {
        let ext: std::string::String =
            match Path::new(&path_string).extension().and_then(OsStr::to_str) {
                Some(p) => p.to_string(),
                None => "".to_string(),
            };

        if option {
            println!(
                "🐱 {0: <18} {1: <18} {2: <18}",
                path_string,
                fs::metadata(&path_string).unwrap().len(),
                ext
            );
        } else {
            println!("🐱 {0: <18}", path_string);
        }
    }
}

// pub fn cutter(path: &String) -> String {
//     let array: Vec<char> = path.chars().collect();

//     array[2..]
//         .into_iter()
//         .map(|_temp| _temp.to_string())
//         .collect::<Vec<String>>()
//         .join("")
// }
