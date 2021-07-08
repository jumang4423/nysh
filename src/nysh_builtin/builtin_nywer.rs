// command line tools
// display images
use viuer::{print_from_file, Config};

pub fn builtin_nywer(file: String) {

    if file == "" {
        println!("give me at least one arg.");
        return;
    }

    let conf = Config {
        // set dimensions
        width: Some(70),
        height: Some(35),
        ..Default::default()
    };

    // starting from row 4 and column 20,
    // display `img.jpg` with dimensions 80x25 (in terminal cells)
    // note that the actual resolution in the terminal will be 80x50
    match print_from_file(file, &conf) {
        Ok(_) => return,
        Err(_) => println!("no file detected.")
    }

}
