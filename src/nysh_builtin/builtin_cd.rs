use dirs;
use std::env;

use super::super::tools::parser::CommandParser;

pub fn builtin_cd(commands: &CommandParser) -> Result<(), String> {
    match commands.args.len() {
        0 => {
            env::set_current_dir(dirs::home_dir().unwrap()).unwrap();
        }
        1 => {
            if !env::set_current_dir(commands.args.join("")).is_ok() {
                return Err("no directory found".to_owned());
            }
        }
        _ => {
            return Err("too many arguments".to_owned());
        }
    }
    Ok(())
}
