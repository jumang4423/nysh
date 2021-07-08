// command line tools
// display images
use viuer::{print_from_file, terminal_size, Config};

pub fn builtin_nywer(args: Vec<String>) {
    if args.len() != 1 {
        println!("give me at least one arg.");
        return;
    }

    let (term_width, _) = terminal_size();

    let conf = Config {
        // set dimensions
        x: (term_width / 2) - (term_width / 4),
        y: 1,
        width: Some(term_width as u32 / 2),
        height: Some(term_width as u32 / 16),
        ..Default::default()
    };

    // starting from row 4 and column 20,
    // display `img.jpg` with dimensions 80x25 (in terminal cells)
    // note that the actual resolution in the terminal will be 80x50

    print!("\x1B[2J\x1B[1;1H");
    match print_from_file(args[0].clone(), &conf) {
        Ok(_) => println!(""),
        Err(_) => println!("no file detected."),
    }
}
