// command line tools
use std::process::Command;

pub fn builtin_la() {
    // wait
    let output = Command::new("ls")
        .arg("-al")
        .output()
        .expect("nysh: command not found");
    print!("{}", String::from_utf8_lossy(&output.stdout));
}
