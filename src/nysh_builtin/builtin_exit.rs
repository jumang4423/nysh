use console::Emoji;
use futures::executor::block_on;
use std::process::exit;

pub async fn exit_process() {
    println!("-! bye{}", Emoji("🐶", ""));
    exit(1);
}

pub fn builtin_exit() {
    println!("-! killing process...");
    block_on(exit_process());
}
