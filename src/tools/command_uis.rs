use std::io;

pub fn get_emoji(emoji_number: usize) -> String {
    let emojis = ["💓", "👀", "🚗", "💰", "🍔", "🌸"];
    String::from(emojis[emoji_number])
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
        .filter(|_temp: &&str| match _temp.chars().next() {
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
