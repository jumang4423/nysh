use std::process::exit;
use futures::executor::block_on;

pub async fn exit_process() {
    println!("-! bye!");
    exit(1);
}

pub fn builtin_exit() {
    println!("-! killing process...");
    block_on(exit_process());
}
