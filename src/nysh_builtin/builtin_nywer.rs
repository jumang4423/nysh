// command line tools
// display images
use viuer::{print_from_file, terminal_size, Config};

pub fn builtin_nywer(file: String) {
    if file == "" {
        println!("give me at least one arg.");
        return;
    }

    let (term_width, term_height) = terminal_size();

    let conf = Config {
        // set dimensions
        width: Some(term_width as u32),
        height: Some(term_height as u32),
        ..Default::default()
    };

    // starting from row 4 and column 20,
    // display `img.jpg` with dimensions 80x25 (in terminal cells)
    // note that the actual resolution in the terminal will be 80x50

    print!("\x1B[2J\x1B[1;1H");
    match print_from_file(file, &conf) {
        Ok(_) => return,
        Err(_) => println!("no file detected."),
    }
}
