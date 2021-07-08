// OverAllTools Object //

// - libraries
use colored::*;
use std::io;
use whoami;
// display images
use viuer::{print_from_file, terminal_size, Config};

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
    let (term_width, _) = terminal_size();

    let conf = Config {
        // set dimensions
        x: (term_width / 2) - (term_width / 4), // (term_width / 2) - (term_width / 6)
        y: 1,
        width: Some(term_width as u32 / 2),
        height: Some(term_width as u32 / 18),
        ..Default::default()
    };
    // starting from row 4 and column 20,
    // display `img.jpg` with dimensions 80x25 (in terminal cells)
    // note that the actual resolution in the terminal will be 80x50

    let home_path: std::path::PathBuf = dirs::home_dir().unwrap();

    print_from_file(
        format!("{}/{}", home_path.clone().display(), ".nysh/_img/lain.jpeg"),
        &conf,
    )
    .expect("Image printing failed.");

    Ok(())
}
