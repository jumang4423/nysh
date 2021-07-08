// OverAllTools Object //

// - libraries
use colored::*;
use std::io;
use whoami;
// display images
use viuer::{print_from_file, Config};

// - welcome func
pub fn say_welcome() -> Result<(), io::Error> {
    let blue_color_res: Result<Color, ()> = "magenta".parse();
    println!(
        "-> welcome to nysh! is the shell written in {}",
        "rust".color(blue_color_res.unwrap_or(Color::Green)).bold(),
    );
    Ok(())
}

// - login name func
pub fn logined_as() -> Result<(), io::Error> {
    let blue_color_res: Result<Color, ()> = "magenta".parse();
    println!(
        "-! logined as {} currently",
        whoami::username()
            .color(blue_color_res.unwrap_or(Color::Green))
            .bold()
    );
    Ok(())
}

pub fn logo_display() -> Result<(), io::Error> {
    let conf = Config {
        // set dimensions
        width: Some(75),
        height: Some(8),
        ..Default::default()
    };

    // starting from row 4 and column 20,
    // display `img.jpg` with dimensions 80x25 (in terminal cells)
    // note that the actual resolution in the terminal will be 80x50
    print_from_file("_img/bk.png", &conf).expect("Image printing failed.");

    Ok(())
}
