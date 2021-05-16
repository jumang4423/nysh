// command line tools
use std::process::Command;
use std::process::Stdio;

pub fn builtin_la() {
    // wait
    let output = Command::new("ls")
        .arg("-al")
        .stdout(Stdio::piped())
        .output()
        .expect("ls not found");
    print!("{}", String::from_utf8_lossy(&output.stdout));

    // match output {
    //     Ok(d) => {
    //         print!("{}", String::from_utf8_lossy(&output.stdout));
    //         Ok(())
    //     }
    //     Err(_) => println!("-! {} ls command not found...", "ops!".red()),
    // }
}
