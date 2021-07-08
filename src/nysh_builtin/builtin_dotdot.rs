// use super::super::nysh_builtin::builtin_cd;
use super::super::tools::parser::CommandParser;

pub fn builtin_dotdot(commands: &mut CommandParser) {
    commands.command = "cd".to_string();
    let _args: Vec<String> = vec!["..".to_string()];
    commands.args = _args;
}
