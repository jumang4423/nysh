use std::fs;
use std::io::*;
use std::str::from_utf8;
use super::super::tools::parser::CommandParser;
use super::super::tools::runner::CommandRunner;

pub async fn builtin_dream95() {
    let password = scan_me("? enter password (length<=6):");
    let hentai = scan_me("? choose your secret(lol) file:");
    let dir = scan_me("? enter secret directory name:");

    println!("> dream95 processing...");
    print!("> ");

    // make tons of directories (lol)
    fs::create_dir_all(&dir).unwrap();
    dft_folder_maker([].to_vec(), password.chars().count(), &dir);

    println!("");
    // copy hentai file
    println!(
        "! {} folders were created.",
        i32::pow(10, password.chars().count() as u32)
    );
    println!("> your dream at:");
    println!("> {}", put_hentai(&hentai, &dir, &password));

    // put hidden flag
    let mut commands = CommandParser::constructor(format!("touch {}/.flag\n", &dir));
    commands.parse_it();
    let mut runner = CommandRunner::constructor(commands).unwrap();
    runner.async_runner().await;

    println!("> enjoy dream95 bruh :8)");
}

pub fn dft_folder_maker(array: Vec<u8>, count_mut: usize, dir: &String) {
    if count_mut != 0 {
        for i in 0..10 {
            let mut new_array = array.clone();
            new_array.push(i as u8);
            let joined: String = String::from(dir.clone() + &String::from("/"))
                + &new_array
                    .clone()
                    .iter()
                    .map(|_temp| _temp.to_string())
                    .collect::<Vec<String>>()
                    .join("/");

            fs::create_dir_all(joined).unwrap();
            dft_folder_maker(new_array, count_mut - 1, &dir);
        }
    }
}

pub fn put_hentai(hentai_file: &String, dir: &String, password: &String) -> String {
    let joined: String = format!(
        "{}/{}/{}",
        dir,
        &password
            .chars()
            .map(|_temp| _temp.to_string())
            .collect::<Vec<String>>()
            .join("/"),
        hentai_file
    );
    match fs::copy(hentai_file, &joined) {
        Ok(_) => println!("file transfered correctly"),
        Err(_) => println!("! no file or folder"),
    }

    joined
}

pub fn scan_me(print_data: &str) -> String {
    print!("{}", print_data);
    stdout().flush().unwrap();
    let mut _scaned_string = String::new();
    stdin()
        .read_line(&mut _scaned_string)
        .expect("input error => ?");

    _scaned_string =
        String::from(from_utf8(&_scaned_string.replace("\n", "").into_bytes()).unwrap());
    return _scaned_string;
}
